from aristech_stt_client import SttClient, NLPProcessRequest, NLPSpec, NLPFunctionSpec
from utils import host, auth_token, auth_secret, root_cert, ssl, server_config, pipeline

client = SttClient(host=host, ssl=ssl, root_cert=root_cert, auth_token=auth_token, auth_secret=auth_secret)

text = "thanks for choosing aristech"
response = client.nlp_process(NLPProcessRequest(
  text="thanks for choosing aristech",
  nlp=NLPSpec(
    server_config=server_config,
    functions=[NLPFunctionSpec(id=func) for func in pipeline]
  )
))
print(response.text)