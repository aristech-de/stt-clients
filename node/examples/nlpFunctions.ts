import 'dotenv/config'

import { SttClient } from '@aristech-org/stt-client'

const auth = process.env.TOKEN && process.env.SECRET ? { token: process.env.TOKEN, secret: process.env.SECRET } : undefined

const client = new SttClient({
  host: process.env.HOST,
  ssl: Boolean(auth) || Boolean(process.env.ROOT_CERT) || process.env.SSL === 'true',
  rootCert: process.env.ROOT_CERT,
  auth,
})
const response = await client.listNlpFunctions()

for (const server of response.server) {
  const { serverConfig, functions } = server
  console.log(`Functions for server ${serverConfig}:`, functions)
}

if (!response.server.length) {
  console.log('No NLP servers available')
}