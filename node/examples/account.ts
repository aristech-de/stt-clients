import 'dotenv/config'

import { SttClient } from '@aristech-org/stt-client'

const client = new SttClient()
const info = await client.accountInfo()
console.log(info)