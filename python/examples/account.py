from aristech_stt_client import SttClient
from dotenv import load_dotenv

load_dotenv()
client = SttClient()
info = client.account_info()
print(info)