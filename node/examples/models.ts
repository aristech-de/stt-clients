import 'dotenv/config'

import { SttClient, endpointingTypeToJSON, grammarTypeToJSON, modelTypeToJSON } from '@aristech-org/stt-client'

const client = new SttClient()
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