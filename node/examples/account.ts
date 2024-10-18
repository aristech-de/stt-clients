import { SttClient } from '@aristech-org/stt-client'
import { getClientOptions } from './utils.js'

const options = getClientOptions()
const client = new SttClient(options)
const info = await client.accountInfo()
console.log(info)