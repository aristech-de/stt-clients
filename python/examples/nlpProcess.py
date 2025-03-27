import os
from aristech_stt_client import SttClient, NLPProcessRequest, NLPSpec, NLPFunctionSpec
from dotenv import load_dotenv

load_dotenv()
server_config = os.getenv("NLP_SERVER_CONFIG", "default")
pipeline = os.getenv("NLP_PIPELINE", "'spellcheck-de")
pipeline = pipeline.split(",")

client = SttClient()

text = "thanks for choosing aristech"
response = client.nlp_process(NLPProcessRequest(
  text="thanks for choosing aristech",
  nlp=NLPSpec(
    server_config=server_config,
    functions=[NLPFunctionSpec(id=func) for func in pipeline]
  )
))
print(response.text)