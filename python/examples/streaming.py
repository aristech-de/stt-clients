import os
import sys
import wave

from aristech_stt_client import SttClient, RecognitionConfig, RecognitionSpec
from utils import host, auth_token, auth_secret, root_cert, ssl, model

# Get the repository root
repo_root = os.path.dirname(os.path.dirname(os.path.abspath(__file__ + "/..")))

client = SttClient(host=host, ssl=ssl, root_cert=root_cert, auth_token=auth_token, auth_secret=auth_secret)

# Load test.wav from the repository root or the first argument
file_path = sys.argv[1] if len(sys.argv) > 1 else os.path.join(repo_root, "test.wav")
# Get the sample rate of the audio file
sample_rate_hertz = wave.open(file_path, "rb").getframerate()

# This function yields the audio data from the file in chunks of 1 second
def audio_data():
  with open(file_path, "rb") as f:
    f.read(44)
    while True:
      data = f.read(sample_rate_hertz)
      if not data:
        break
      yield data

# Recognize the audio file
it = client.recognize(
  RecognitionConfig(
    specification=RecognitionSpec(
      sample_rate_hertz=sample_rate_hertz,
      model=model,
      partial_results=True,
      single_utterance=False
    )
  ),
  audio_data(),
)
for result in it:
  chunk = result.chunks[0]
  alternative = chunk.alternatives[0]
  # Clear the current line
  print("\033[K", end="\r")
  # Print the current transcription and a newline if it's the end of the utterance
  eou = chunk.end_of_utterance or chunk.final
  print(alternative.text, end=eou and "\n" or "")