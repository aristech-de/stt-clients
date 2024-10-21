import 'dotenv/config'

import { SttClient } from '@aristech-org/stt-client'

const auth = process.env.TOKEN && process.env.SECRET ? { token: process.env.TOKEN, secret: process.env.SECRET } : undefined

const client = new SttClient({
  host: process.env.HOST,
  ssl: Boolean(auth) || Boolean(process.env.ROOT_CERT) || process.env.SSL === 'true',
  rootCert: process.env.ROOT_CERT,
  auth,
})

// To see which server configs and functions within those servers are available, run the nlpFunctions.ts example
const serverConfig = process.env.NLP_SERVER_CONFIG || 'default'
const pipeline = process.env.NLP_PIPELINE || 'spellcheck-de'
const functions = pipeline.split(',').map(f => ({ id: f }))

const text = 'thanks for choosing aristech'
const response = await client.nlpProcess({
  text,
  nlp: {
    serverConfig,
    functions,
  }
})

console.log('NLP processing response:', response.text)