# Aristech STT-Client for NodeJS

This is the NodeJS client implementation for the Aristech STT-Server.

## Installation

```bash
npm install @aristech-org/stt-client
```

## Usage

```typescript
import { SttClient } from '@aristech-org/stt-client'

const client = new SttClient({ host: 'stt.example.com' })
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

You can run the examples directly using `tsx` like this:

1. Create a `.env` file in the [node](.) directory:

```sh
HOST=stt.example.com
# The credentials are optional but probably required for most servers:
TOKEN=your-token
SECRET=your-secret

# The following are optional:
# ROOT_CERT=your-root-cert.pem # If the server uses a self-signed certificate
# SSL=true # Set to true if credentials are provided or if a ROOT_CERT is provided
# MODEL=some-available-model
# NLP_SERVER=some-config
# NLP_PIPELINE=function1,function2
```

2. Run the examples, e.g.:

```sh
npx tsx examples/streaming.ts
```

## Build

To rebuild the generated typescript files from the proto file, run:

```bash
npm run generate
```

To build the library, run:

```bash
npm run build
```

