import 'dotenv/config'

import { SttClient } from '@aristech-org/stt-client'

const client = new SttClient()

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