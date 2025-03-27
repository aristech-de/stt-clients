import grpc
import re
import wave
import base64
import os

from typing import Iterable

from .proto.stt_service_pb2_grpc import *
from .proto.stt_service_pb2 import *
class SttClient:
    host: str
    ssl: bool
    rootCert: bytes
    auth_token: str
    auth_secret: str
    channel: grpc.Channel

    def __init__(self, host="", ssl=None, root_cert=b"", auth_token="", auth_secret="", api_key=os.getenv("ARISTECH_STT_API_KEY", "")):
      """
      Initializes the client with the given connection parameters.

      Args:
        host (str): The host to connect to. Can include the port, e.g. "localhost:9424".
        ssl (bool): Whether to use SSL. If not explicitly set, the client will try to guess based on the remaining parameters.
        root_cert (bytes): The PEM encoded root certificate to use for SSL (e.g. when connecting to a server that uses a self-signed certificate).
        auth_token (str): The auth token to use for authentication.
        auth_secret (str): The auth secret to use for authentication.
      """
      # When ssl or rootCert are not explicitly set, we check if the host includes the port 9424 or 9423.
      # If host does not include the port, we assume ssl is True and the port is 9424 therefore.
      defaultSsl = (ssl is None and len(root_cert) == 0) or (ssl is True or len(root_cert) != 0) or len(api_key) != 0

      self.host = ""
      if len(host) != 0:
        (h, p) = self._get_host_port(host, defaultSsl)
        self.host = h + ":" + p
      self.rootCert = root_cert

      # Check if the root certificate is set in the environment
      if len(self.rootCert) == 0:
        env_root_cert=os.getenv("ARISTECH_STT_CA_CERTIFICATE", "")
        if len(env_root_cert) != 0:
          self.rootCert = open(env_root_cert, "rb").read()

      if len(api_key) != 0:
        (auth_token, auth_secret, key_host) = self._decode_api_key(api_key)
        if len(self.host) == 0 and len(key_host) != 0:
          self.host = key_host
        self.auth_token = auth_token
        self.auth_secret = auth_secret
        self.ssl = True
        self.channel = self._create_secure_channel()
        return

      self.ssl = ssl is True or len(root_cert) != 0 or p == "9424"
      self.auth_token = auth_token
      self.auth_secret = auth_secret
      if self.ssl or self.rootCert:
        self.channel = self._create_secure_channel()
      else:
        self.channel = grpc.insecure_channel(self.host)
    
    def _decode_api_key(self, api_key):
      # Remove the "at-" prefix
      api_key = api_key[3:]
      base64url = api_key.replace("-", "+").replace("_", "/")
      padding = '=' * (-len(base64url) % 4)
      base64url += padding
      decoded_bytes = base64.b64decode(base64url)
      key_data = decoded_bytes.decode('utf-8')
      # The key is a yaml file with token and secret and an optional host
      key_data = key_data.split("\n")
      token = ""
      secret = ""
      host = ""
      for line in key_data:
        key_value = line.split(":", 1)
        if len(key_value) != 2:
          continue
        k = key_value[0].strip()
        v = key_value[1].strip()
        if k == "token":
          token = v
        elif k == "secret":
          secret = v
        elif k == "host":
          host = v
        elif k == "type":
          if v != "stt":
            raise ValueError("The API key is not for the STT service but for " + v.upper())

      return (token, secret, host)
          
    
    def _get_host_port(self, host, defaultSsl):
      portRe = r"^(?P<host>[^:]+):(?P<port>[0-9]+)$"
      matches = re.search(portRe, host)
      defaultPort = defaultSsl and "9424" or "9423"
      return (host, defaultPort) if matches is None else (matches.group("host"), matches.group("port"))
    
    def _metadata_callback(self, context, callback):
      callback([('token', self.auth_token), ('secret', self.auth_secret)], None)

    def _create_secure_channel(self):
        if len(self.rootCert) != 0:
          cert_creds = grpc.ssl_channel_credentials(root_certificates=self.rootCert)
        else:
          cert_creds = grpc.ssl_channel_credentials()
        auth_creds = grpc.metadata_call_credentials(self._metadata_callback)
        combined_creds = grpc.composite_channel_credentials(cert_creds, auth_creds)
        channel = grpc.secure_channel(target=self.host, credentials=combined_creds)
        return channel
    
    def _file_request_generator(self, file_path: str, config: RecognitionConfig):
      # Set partial_results to false explicitly as we don't need partial results because we stream the whole file at once.
      config.specification.partial_results = False
      # Get the sample rate from the file
      with wave.open(file_path, "rb") as f:
        config.specification.sample_rate_hertz = f.getframerate()
      # Tell the server what we're sending and how to process it
      yield StreamingRecognitionRequest(config=config)
      # Send the audio file
      data = open(file_path, "rb").read()
      # Skip the header of the wav file
      data = data[44:]
      yield StreamingRecognitionRequest(audio_content=data)
    
    def _streaming_request_generator(self, config: RecognitionConfig, audio_stream: Iterable[bytes]):
      # Tell the server what we're sending and how to process it
      yield StreamingRecognitionRequest(config=config)
      # Send the audio data
      for data in audio_stream:
        yield StreamingRecognitionRequest(audio_content=data)

    def list_models(self, request=ModelsRequest()) -> ModelsResponse:
      """
      List available models.

      Args:
        request (ModelsRequest): The optional request object.
      """
      stub = SttServiceStub(self.channel)
      return stub.Models(request)
  
    def account_info(self, request=AccountInfoRequest()) -> AccountInfoResponse:
      """
      Get account information.

      Args:
        request (AccountInfoRequest): The optional request object.
      """
      stub = SttServiceStub(self.channel)
      return stub.AccountInfo(request)
    
    def list_nlp_functions(self, request=NLPFunctionsRequest()) -> NLPFunctionsResponse:
      """
      List available NLP functions.

      Args:
        request (NLPFunctionsRequest): The optional request object.
      """
      stub = SttServiceStub(self.channel)
      return stub.NLPFunctions(request)
    
    def nlp_process(self, request: NLPProcessRequest) -> NLPProcessResponse:
      """
      Process text with NLP.

      Args:
        request (NLPProcessRequest): The request object.
      """
      stub = SttServiceStub(self.channel)
      return stub.NLPProcess(request)

    def recognize_file(self, file_path: str, config: RecognitionConfig) -> Iterable[StreamingRecognitionResponse]:
      """
      Recognize a wave file.

      Args:
        file_path (str): The path to the wave file.
        config (RecognitionConfig): The recognition configuration.

      Returns:
        Iterable[StreamingRecognitionResponse]: The recognition results.
      """
      stub = SttServiceStub(self.channel)
      return stub.StreamingRecognize(self._file_request_generator(file_path, config))
    
    def recognize(self, config: RecognitionConfig, audio_stream: Iterable[bytes]) -> Iterable[StreamingRecognitionResponse]:
      """
      Recognize audio data from a stream.

      Args:
        config (RecognitionConfig): The recognition configuration.
        audio_stream (Iterable[bytes]): The audio stream.

      Returns:
        Iterable[StreamingRecognitionResponse]: The recognition results.
      """
      stub = SttServiceStub(self.channel)
      return stub.StreamingRecognize(self._streaming_request_generator(config, audio_stream))