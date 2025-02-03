// Run: npx tsx examples/streaming.ts [path to wave file]
import 'dotenv/config'

import { StreamingRecognitionResponse, SttClient, getWaveSampleRate } from '@aristech-org/stt-client'

import fs from 'fs'
import path from 'path'

// Use the second command line argument as the path to the wave file
let fileName = process.argv[2]
if (!fileName) {
  // Fallback to test.wav in the repository root
  const scriptDir = path.dirname(new URL(import.meta.url).pathname)
  fileName = path.join(scriptDir, '../../test.wav')
} else if (!fs.existsSync(fileName) || !fileName.endsWith('.wav')) {
  console.error(`The file ${fileName} does not exist or does not end with .wav`)
  process.exit(1)
}

// Create a client and a recognition stream
const auth = process.env.TOKEN && process.env.SECRET ? { token: process.env.TOKEN, secret: process.env.SECRET } : undefined
const client = new SttClient({
  host: process.env.HOST,
  ssl: Boolean(auth) || Boolean(process.env.ROOT_CERT) || process.env.SSL === 'true',
  rootCert: process.env.ROOT_CERT,
  auth,
})
const stream = client.recognize({
  specification: {
    // When streaming, we need to provide the sample rate, because the data are expected to be raw PCM data without a header
    sampleRateHertz: getWaveSampleRate(fileName),
    partialResults: true,
    model: process.env.MODEL || 'generic-small-en-us-0.15',
    // The locale is optional and only relevant for multilingual models which autodetect the language if not specified explicitly.
    locale: process.env.LOCALE || '', // e.g. 'en
    // Optional - Can be used for multilingual models to add a glossary with task-specific vocabulary (e.g. product names) that would otherwise be unlikely to be recognized. Can also be used to steer the recognition towards a specific writing style.
    prompt: process.env.MODEL_PROMPT,
  }
})

// Log the recognition results
stream.on('data', (result: StreamingRecognitionResponse) => {
  const chunk = result.chunks[0] // This is typically always one chunk
  if (!chunk) return
  const alt = chunk.alternatives[0]
  if (!alt) return

  // Final indicates that the whole stream has been processed,
  // endOfUtterance indicates that this result is the end of an utterance
  const isPartial = !chunk.final && !chunk.endOfUtterance
  
  // Clear the line and write the new text
  process.stdout.clearLine(0)
  process.stdout.cursorTo(0)
  process.stdout.write(alt.text)
  if (!isPartial) {
    process.stdout.write('\n')
  }
})

// Get the wave file and start streaming it
const waveFile = fs.createReadStream(fileName)
// Skip the wave header (which is typically 44 bytes)
waveFile.read(44)
// We have to create a StreamingRecognitionRequest for each audio chunk
waveFile.on('data', (chunk: string | Buffer<ArrayBufferLike>) => {
  const audioContent = typeof chunk === 'string' ? Uint8Array.from(Buffer.from(chunk)) : Uint8Array.from(chunk)
  stream.write({ audioContent })
})
// When the wave file ends, half-close the stream to signal the end of the audio
waveFile.on('end', () => {
  stream.end()
})