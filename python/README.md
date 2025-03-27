# Aristech STT-Client for Python

This is the Python client implementation for the Aristech STT-Server.

## Installation

```bash
pip install aristech-stt-client
```

## Usage

```python
from aristech_stt_client import SttClient, RecognitionConfig, RecognitionSpec

client = SttClient(
  api_key=os.getenv('ARISTECH_STT_API_KEY', ''), # This is the default and can be omitted
)
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

To run the examples while using the local version of the package, run the package with the `PYTHONPATH` environment variable set to the src directory:

```sh
PYTHONPATH=src python examples/streaming.py
```

### API Key

If you didn't get an API key but a token, secret and host instead, you can simply convert those values with our [API key generator](https://www.aristech.de/api-key-generator/?type=stt).

<details>

<summary>Alternatively you can still provide the connection options manually.</summary>

```python
from aristech_stt_client import SttClient

client = SttClient(host='stt.example.com:443', auth_token='your-token', auth_secret='your-secret')
```

</details>
