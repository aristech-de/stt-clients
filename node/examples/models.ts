import 'dotenv/config'

import { SttClient, endpointingTypeToJSON, grammarTypeToJSON, modelTypeToJSON } from '@aristech-org/stt-client'

const auth = process.env.TOKEN && process.env.SECRET ? { token: process.env.TOKEN, secret: process.env.SECRET } : undefined

const client = new SttClient({
  host: process.env.HOST,
  ssl: Boolean(auth) || Boolean(process.env.ROOT_CERT) || process.env.SSL === 'true',
  rootCert: process.env.ROOT_CERT,
  auth,
})
const models = await client.listModels()

for (const model of models) {
  // We pull out the enum values and convert them to their string representation for better readability
  const { grammarType, type, endpointing, ...rest } = model
  console.log({
    ...rest,
    type: modelTypeToJSON(type),
    grammarType: grammarType.map((grammarType) => grammarTypeToJSON(grammarType)),
    endpointing: endpointing.map((endpointingType) => endpointingTypeToJSON(endpointingType)),
  })
}