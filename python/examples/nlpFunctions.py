from aristech_stt_client import SttClient
from dotenv import load_dotenv

load_dotenv()
client = SttClient()
response = client.list_nlp_functions()
print(response)