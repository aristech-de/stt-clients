# Aristech STT-Client for Python

This is the Python client implementation for the Aristech STT-Server.

## Installation

```bash
pip install aristech-stt-client
```

## Usage

```python
from aristech_stt_client import SttClient, RecognitionConfig, RecognitionSpec

client = SttClient(host='stt.example.com')
results = client.recognize_file("path/to/audio/file.wav", RecognitionConfig(specification=RecognitionSpec(model="some-model")))
print('\n'.join([r.chunks[0].alternatives[0].text for r in results]))
```

There are several examples in the [examples](https://github.com/aristech-de/stt-clients/blob/main/python/examples/) directory:

- [recognize.py](https://github.com/aristech-de/stt-clients/blob/main/python/examples/recognize.py): Demonstrates how to perform recognition on a file.
- [streaming.py](https://github.com/aristech-de/stt-clients/blob/main/python/examples/streaming.py): Demonstrates how to stream audio to the server while receiving interim results.
- [models.py](https://github.com/aristech-de/stt-clients/blob/main/python/examples/models.py): Demonstrates how to get the available models from the server.
- [nlpFunctions.py](https://github.com/aristech-de/stt-clients/blob/main/python/examples/nlpFunctions.py): Demonstrates how to list the configured NLP-Servers and the coresponding functions.
- [nlpProcess.py](https://github.com/aristech-de/stt-clients/blob/main/python/examples/nlpProcess.py): Demonstrates how to perform NLP processing on a text by using the STT-Server as a proxy.
- [account.py](https://github.com/aristech-de/stt-clients/blob/main/python/examples/account.py): Demonstrates how to retrieve the account information from the server.

You can run the examples directly using `python` like this:

1. Create a `.env` file in the [python](.) directory:

```sh
HOST=stt.example.com
# The credentials are optional but probably required for most servers:
TOKEN=your-token
SECRET=your-secret

# The following are optional:
# ROOT_CERT=your-root-cert.pem # If the server uses a self-signed certificate
# MODEL=some-available-model
# NLP_SERVER=some-config
# NLP_PIPELINE=function1,function2
```

2. Run the examples, e.g.:

```sh
python examples/streaming.py
```
