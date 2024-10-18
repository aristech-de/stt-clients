# Aristech STT-Client for NodeJS

This is the NodeJS client implementation for the Aristech STT-Server.

## Installation

```bash
npm install @aristech-org/stt-client
```

## Usage

```typescript
import { SttClient, RecognitionConfig } from '@aristech-org/stt-client';

const client = new SttClient('localhost:9424');
const result = await client.recognize('path/to/audio/file.wav')
console.log(result.chunks[0].alternatives[0].text);
```

There are several examples in the `examples` directory:

- [recognize.ts](examples/recognize.ts): Pretty much the same as the example above.
- [streaming.ts](examples/streaming.ts): Demonstrates how to stream audio to the server while receiving interim results.
- [models.ts](examples/models.ts): Demonstrates how to get the available models from the server.
- [nlpFunctions.ts](examples/nlpFunctions.ts): Demonstrates how to list the configured NLP-Servers and the coresponding functions.
- [nlpProcess.ts](examples/nlpProcess.ts): Demonstrates how to perform NLP processing on a text by using the STT-Server as a proxy.
- [account.ts](examples/account.ts): Demonstrates how to retrieve the account information from the server.

## Build

To rebuild the generated typescript files from the proto file, run the following command:

```bash
npm run generate
```

To build the project, run the following command:

```bash
npm run build
```
