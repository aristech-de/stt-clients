import { getClientOptions, getWaveFilePath } from './utils.js'

import { SttClient } from '@aristech-org/stt-client'

const client = new SttClient(getClientOptions())
const result = await client.recognizeFile(getWaveFilePath())
console.log(result.chunks[0].alternatives[0].text)