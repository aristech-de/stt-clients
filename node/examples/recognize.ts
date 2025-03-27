// Run: npx tsx examples/recognize.ts [path to wave file]
import 'dotenv/config'

import { SttClient } from '@aristech-org/stt-client'
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

const client = new SttClient()
const results = await client.recognizeFile(fileName, { specification: { model: process.env.MODEL } })
console.log(results.map(r => r.chunks[0].alternatives[0].text).join('\n'))