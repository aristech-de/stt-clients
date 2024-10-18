import { SttClient } from '@aristech-org/stt-client'
import { getClientOptions } from './utils.js'

const client = new SttClient(getClientOptions())
const response = await client.listNlpFunctions()

for (const server of response.server) {
  const { serverConfig, functions } = server
  console.log(`Functions for server ${serverConfig}:`, functions)
}