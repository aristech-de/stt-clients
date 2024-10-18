import { NLPProcessRequest, SttClient } from '@aristech-org/stt-client'
import { getArg, getClientOptions } from './utils.js'

const client = new SttClient(getClientOptions())

const text = 'thanks for choosing aristech'
const response = await client.nlpProcess(NLPProcessRequest.create({
  text,
  nlp: {
    // To see which server configs and functions within those servers are available, run `npx tsx examples/listNlpFunctions.ts`
    serverConfig: getArg('config', 'default'),
    functions: getArg('functions', 'spellcheck-de').split(';').map(f => ({ id: f })),
  }
}))

console.log('NLP processing response:', response.text)