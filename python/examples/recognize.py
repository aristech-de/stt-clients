import os
import sys

from aristech_stt_client import SttClient, RecognitionConfig, RecognitionSpec
from utils import host, auth_token, auth_secret, root_cert, ssl, model

# Get the repository root
repo_root = os.path.dirname(os.path.dirname(os.path.abspath(__file__ + "/..")))

client = SttClient(host=host, ssl=ssl, root_cert=root_cert, auth_token=auth_token, auth_secret=auth_secret)

# Load test.wav from the repository root or the first argument
# We do not have to specify the sample rate here because this is figured out by the client when using the recognize_file method
file_path = sys.argv[1] if len(sys.argv) > 1 else os.path.join(repo_root, "test.wav")
results = client.recognize_file(file_path, RecognitionConfig(specification=RecognitionSpec(model=model)))
print('\n'.join([r.chunks[0].alternatives[0].text for r in results]))