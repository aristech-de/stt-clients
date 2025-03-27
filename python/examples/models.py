from aristech_stt_client import SttClient
from dotenv import load_dotenv

load_dotenv()
client = SttClient()
models = client.list_models()
print(models)