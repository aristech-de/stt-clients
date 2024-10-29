from aristech_stt_client import SttClient
from utils import host, auth_token, auth_secret, root_cert, ssl

client = SttClient(host=host, ssl=ssl, root_cert=root_cert, auth_token=auth_token, auth_secret=auth_secret)
models = client.list_models()
print(models)