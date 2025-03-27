# Aristech STT-Client for NodeJS

This is the NodeJS client implementation for the Aristech STT-Server.

## Installation

```bash
npm install @aristech-org/stt-client
```

## Usage

```typescript
import { SttClient } from '@aristech-org/stt-client'

const client = new SttClient({
  apiKey: process.env['ARISTECH_STT_API_KEY'], // This is the default and can be omitted
})
const results = await client.recognizeFile('path/to/audio/file.wav')
console.log(results.map(r => r.chunks[0].alternatives[0].text).join('\n'))
```

There are several examples in the [examples](https://github.com/aristech-de/stt-clients/blob/main/node/examples/) directory:

- [recognize.ts](https://github.com/aristech-de/stt-clients/blob/main/node/examples/recognize.ts): Pretty much the same as the example above.
- [streaming.ts](https://github.com/aristech-de/stt-clients/blob/main/node/examples/streaming.ts): Demonstrates how to stream audio to the server while receiving interim results.
- [models.ts](https://github.com/aristech-de/stt-clients/blob/main/node/examples/models.ts): Demonstrates how to get the available models from the server.
- [nlpFunctions.ts](https://github.com/aristech-de/stt-clients/blob/main/node/examples/nlpFunctions.ts): Demonstrates how to list the configured NLP-Servers and the coresponding functions.
- [nlpProcess.ts](https://github.com/aristech-de/stt-clients/blob/main/node/examples/nlpProcess.ts): Demonstrates how to perform NLP processing on a text by using the STT-Server as a proxy.
- [account.ts](https://github.com/aristech-de/stt-clients/blob/main/node/examples/account.ts): Demonstrates how to retrieve the account information from the server.

To run the examples, use `npx tsx`. For example:

```sh
npx tsx examples/streaming.ts
```

### API Key

If you didn't get an API key but a token, secret and host, you can generate an API key with our [API key generator](https://www.aristech.de/api-key-generator/?type=stt).

<details>

<summary>Alternatively you can still provide the connection options manually.</summary>

```typescript
import { SttClient } from '@aristech-org/stt-client'

const client = new SttClient({
  host: 'stt.example.com:443',
  auth: {
    token: 'your-token',
    secret: 'your-secret'
  }
})
```
</details>

## Build

To rebuild the generated typescript files from the proto file, run:

```bash
npm run generate
```

To build the library, run:

```bash
npm run build
```

