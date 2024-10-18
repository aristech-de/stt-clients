// Usage: npx tsx examples/recognize.ts path/to/audio.wav [sampleRate]
import { RecognitionConfig, StreamingRecognitionRequest, StreamingRecognitionResponse, SttClient, getWaveSampleRate } from '@aristech-org/stt-client'
import { getArg, getClientOptions, getWaveFile, getWaveFilePath } from './utils.js'

// Create a client and a recognition stream
const client = new SttClient(getClientOptions())
const recognitionConfig = RecognitionConfig.create({
  specification: {
    sampleRateHertz: getWaveSampleRate(getWaveFilePath()),
    partialResults: true,
    model: getArg('model', 'generic-small-en-us-0.15'),
  }
})
const stream = client.recognize(recognitionConfig)

// Log the recognition results
stream.on('data', (result: StreamingRecognitionResponse) => {
  const chunk = result.chunks[0] // This is typically always one chunk
  if (!chunk) return
  const alt = chunk.alternatives[0]
  if (!alt) return

  const finalOrPartial = chunk.final ? 'Final' : 'Partial'
  console.log(`${finalOrPartial}: "${alt.text}"`)
})

// Get the wave file and start streaming it
const waveFile = getWaveFile()
// We have to create a StreamingRecognitionRequest for each audio chunk
waveFile.on('data', (chunk: Buffer) => {
  stream.write(StreamingRecognitionRequest.create({ audioContent: Uint8Array.from(chunk) }))
})
// When the wave file ends, half-close the stream to signal the end of the audio
waveFile.on('end', () => {
  stream.end()
})