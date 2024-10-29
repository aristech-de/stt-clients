# -*- coding: utf-8 -*-
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# NO CHECKED-IN PROTOBUF GENCODE
# source: stt_service.proto
# Protobuf Python Version: 5.27.2
"""Generated protocol buffer code."""
from google.protobuf import descriptor as _descriptor
from google.protobuf import descriptor_pool as _descriptor_pool
from google.protobuf import runtime_version as _runtime_version
from google.protobuf import symbol_database as _symbol_database
from google.protobuf.internal import builder as _builder
_runtime_version.ValidateProtobufRuntimeVersion(
    _runtime_version.Domain.PUBLIC,
    5,
    27,
    2,
    '',
    'stt_service.proto'
)
# @@protoc_insertion_point(imports)

_sym_db = _symbol_database.Default()


from google.protobuf import duration_pb2 as google_dot_protobuf_dot_duration__pb2


DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n\x11stt_service.proto\x12\nari.stt.v1\x1a\x1egoogle/protobuf/duration.proto\"|\n\x1bStreamingRecognitionRequest\x12/\n\x06\x63onfig\x18\x01 \x01(\x0b\x32\x1d.ari.stt.v1.RecognitionConfigH\x00\x12\x17\n\raudio_content\x18\x02 \x01(\x0cH\x00\x42\x13\n\x11streaming_request\"\x96\x01\n\x1cStreamingRecognitionResponse\x12\x32\n\x06\x63hunks\x18\x01 \x03(\x0b\x32\".ari.stt.v1.SpeechRecognitionChunk\x12\x11\n\tclient_id\x18\x03 \x01(\t\x12\x10\n\x08language\x18\x04 \x01(\tJ\x04\x08\x02\x10\x03R\x17\x65nd_of_single_utterance\"G\n\x11RecognitionConfig\x12\x32\n\rspecification\x18\x01 \x01(\x0b\x32\x1b.ari.stt.v1.RecognitionSpec\"\xd3\x03\n\x0fRecognitionSpec\x12\x41\n\x0e\x61udio_encoding\x18\x01 \x01(\x0e\x32).ari.stt.v1.RecognitionSpec.AudioEncoding\x12\x19\n\x11sample_rate_hertz\x18\x02 \x01(\x03\x12\x0e\n\x06locale\x18\x03 \x01(\t\x12\r\n\x05graph\x18\x05 \x01(\t\x12\x0f\n\x07grammar\x18\x06 \x01(\t\x12\x17\n\x0fpartial_results\x18\x07 \x01(\x08\x12\x18\n\x10single_utterance\x18\x08 \x01(\x08\x12\x34\n\rnormalization\x18\t \x01(\x0b\x32\x1d.ari.stt.v1.NormalizationSpec\x12\x0e\n\x06phones\x18\n \x01(\x08\x12\r\n\x05model\x18\x0b \x01(\t\x12-\n\x0b\x65ndpointing\x18\x0c \x01(\x0b\x32\x18.ari.stt.v1.EndpointSpec\x12 \n\x03vad\x18\r \x01(\x0b\x32\x13.ari.stt.v1.VadSpec\"A\n\rAudioEncoding\x12\x1e\n\x1a\x41UDIO_ENCODING_UNSPECIFIED\x10\x00\x12\x10\n\x0cLINEAR16_PCM\x10\x01J\x04\x08\x04\x10\x05R\x10profanity_filter\"n\n\x11NormalizationSpec\x12\x11\n\tstrip_unk\x18\x02 \x01(\x08\x12 \n\x03nlp\x18\x04 \x01(\x0b\x32\x13.ari.stt.v1.NLPSpecJ\x04\x08\x01\x10\x02J\x04\x08\x03\x10\x04R\x0braw_resultsR\x0bstrip_slots\"\xfe\x01\n\x07NLPSpec\x12\x15\n\rserver_config\x18\x01 \x01(\t\x12.\n\tfunctions\x18\x02 \x03(\x0b\x32\x1b.ari.stt.v1.NLPFunctionSpec\x12\x17\n\x0fpartial_results\x18\x03 \x01(\x08\x12\x0c\n\x04\x61rgs\x18\x04 \x01(\t\x12\x36\n\x0binput_field\x18\x05 \x01(\x0e\x32!.ari.stt.v1.NLPSpec.NlpInputField\"M\n\rNlpInputField\x12\x0f\n\x0bUNSPECIFIED\x10\x00\x12\x08\n\x04TEXT\x10\x01\x12\x0f\n\x0bTAGGED_TEXT\x10\x02\x12\x10\n\x0cSLOTTED_TEXT\x10\x03\"+\n\x0fNLPFunctionSpec\x12\n\n\x02id\x18\x01 \x01(\t\x12\x0c\n\x04\x61rgs\x18\x02 \x03(\t\"\xbc\x01\n\x0c\x45ndpointSpec\x12\x17\n\x0fsilence_timeout\x18\x01 \x01(\x02\x12)\n!trailing_silence_high_probability\x18\x02 \x01(\x02\x12\'\n\x1ftrailing_silence_ok_probability\x18\x03 \x01(\x02\x12$\n\x1ctrailing_silence_no_endpoint\x18\x04 \x01(\x02\x12\x19\n\x11utterance_timeout\x18\x05 \x01(\x02\"J\n\x07VadSpec\x12\x11\n\tthreshold\x18\x01 \x01(\x02\x12\x18\n\x10trailing_silence\x18\x02 \x01(\x02\x12\x12\n\nmin_speech\x18\x03 \x01(\x02\"\x81\x01\n\x16SpeechRecognitionChunk\x12>\n\x0c\x61lternatives\x18\x01 \x03(\x0b\x32(.ari.stt.v1.SpeechRecognitionAlternative\x12\r\n\x05\x66inal\x18\x02 \x01(\x08\x12\x18\n\x10\x65nd_of_utterance\x18\x03 \x01(\x08\"\xb9\x01\n\x1cSpeechRecognitionAlternative\x12\x0c\n\x04text\x18\x01 \x01(\t\x12\x14\n\x0cslotted_text\x18\x07 \x01(\t\x12\x13\n\x0btagged_text\x18\x05 \x01(\t\x12\x10\n\x08nlp_text\x18\x06 \x01(\t\x12\x12\n\nconfidence\x18\x02 \x01(\x02\x12#\n\x05words\x18\x03 \x03(\x0b\x32\x14.ari.stt.v1.WordInfoJ\x04\x08\x04\x10\x05R\x0fnormalized_text\"\xe1\x01\n\x08WordInfo\x12-\n\nstart_time\x18\x01 \x01(\x0b\x32\x19.google.protobuf.Duration\x12+\n\x08\x65nd_time\x18\x02 \x01(\x0b\x32\x19.google.protobuf.Duration\x12\x0c\n\x04word\x18\x03 \x01(\t\x12\x12\n\nconfidence\x18\x04 \x01(\x02\x12%\n\x06phones\x18\x05 \x03(\x0b\x32\x15.ari.stt.v1.PhoneInfo\x12\x0c\n\x04slot\x18\x07 \x01(\tJ\x04\x08\x06\x10\x07J\x04\x08\x08\x10\tR\x08raw_wordR\x0c\x65ntity_label\"v\n\tPhoneInfo\x12-\n\nstart_time\x18\x01 \x01(\x0b\x32\x19.google.protobuf.Duration\x12+\n\x08\x65nd_time\x18\x02 \x01(\x0b\x32\x19.google.protobuf.Duration\x12\r\n\x05phone\x18\x03 \x01(\t\"\x0f\n\rModelsRequest\"2\n\x0eModelsResponse\x12 \n\x05model\x18\x01 \x03(\x0b\x32\x11.ari.stt.v1.Model\"\xa0\x02\n\x05Model\x12\n\n\x02id\x18\x01 \x01(\t\x12\x0c\n\x04name\x18\x08 \x01(\t\x12\x13\n\x0b\x64\x65scription\x18\t \x01(\t\x12\x0f\n\x07version\x18\n \x01(\t\x12#\n\x04type\x18\x02 \x01(\x0e\x32\x15.ari.stt.v1.ModelType\x12\x0e\n\x06locale\x18\x03 \x03(\t\x12-\n\x0cgrammar_type\x18\x04 \x03(\x0e\x32\x17.ari.stt.v1.GrammarType\x12 \n\x03nlp\x18\x05 \x01(\x0b\x32\x13.ari.stt.v1.NLPSpec\x12\r\n\x05slots\x18\x06 \x03(\t\x12\x10\n\x08\x65xamples\x18\x07 \x03(\t\x12\x30\n\x0b\x65ndpointing\x18\x0b \x03(\x0e\x32\x1b.ari.stt.v1.EndpointingType\"\x15\n\x13NLPFunctionsRequest\"E\n\x14NLPFunctionsResponse\x12-\n\x06server\x18\x01 \x03(\x0b\x32\x1d.ari.stt.v1.NLPFunctionServer\"V\n\x11NLPFunctionServer\x12\x15\n\rserver_config\x18\x01 \x01(\t\x12*\n\tfunctions\x18\x03 \x03(\x0b\x32\x17.ari.stt.v1.NLPFunction\"<\n\x0bNLPFunction\x12\n\n\x02id\x18\x01 \x01(\t\x12\x0c\n\x04name\x18\x02 \x01(\t\x12\x13\n\x0b\x64\x65scription\x18\x03 \x01(\t\"\x10\n\x0eLocalesRequest\"5\n\x0fLocalesResponse\x12\"\n\x06locale\x18\x01 \x03(\x0b\x32\x12.ari.stt.v1.Locale\"n\n\x06Locale\x12\x0e\n\x06locale\x18\x01 \x01(\t\x12\x0f\n\x07\x64ynamic\x18\x03 \x01(\x08\x12\r\n\x05model\x18\x04 \x03(\t\x12!\n\x06graphs\x18\x05 \x03(\x0b\x32\x11.ari.stt.v1.GraphJ\x04\x08\x02\x10\x03R\x0bgraph_names\"(\n\x05Graph\x12\x0c\n\x04name\x18\x01 \x01(\tJ\x04\x08\x02\x10\x03R\x0bnormalizers\"\x14\n\x12\x41\x63\x63ountInfoRequest\"\xaa\x01\n\x13\x41\x63\x63ountInfoResponse\x12\r\n\x05token\x18\x01 \x01(\t\x12\x14\n\x0c\x64isplay_name\x18\x02 \x01(\t\x12\x16\n\x0etotal_requests\x18\x03 \x01(\x03\x12\x16\n\x0e\x62ooked_seconds\x18\x04 \x01(\x03\x12\x14\n\x0cused_seconds\x18\x05 \x01(\x03\x12\x17\n\x0f\x65xpiration_date\x18\x06 \x01(\x03\x12\x0f\n\x07\x62locked\x18\x07 \x01(\x08\"C\n\x11NLPProcessRequest\x12\x0c\n\x04text\x18\x01 \x01(\t\x12 \n\x03nlp\x18\x02 \x01(\x0b\x32\x13.ari.stt.v1.NLPSpec\"\"\n\x12NLPProcessResponse\x12\x0c\n\x04text\x18\x01 \x01(\t*\"\n\x0f\x45ndpointingType\x12\x06\n\x02LM\x10\x00\x12\x07\n\x03VAD\x10\x01*L\n\tModelType\x12\x0c\n\x08\x43ORE_STT\x10\x00\x12\x0f\n\x0bGRAMMAR_STT\x10\x01\x12\x0f\n\x0bWHISPER_STT\x10\x02\x12\x0f\n\x0b\x44IARIZATION\x10\x03*;\n\x0bGrammarType\x12\x08\n\x04JSGF\x10\x00\x12\x08\n\x04SRGS\x10\x03\x12\x07\n\x03KWS\x10\x01\x12\x0f\n\x0bPHRASE_LIST\x10\x02\x32\xfd\x03\n\nSttService\x12m\n\x12StreamingRecognize\x12\'.ari.stt.v1.StreamingRecognitionRequest\x1a(.ari.stt.v1.StreamingRecognitionResponse\"\x00(\x01\x30\x01\x12\x41\n\x06Models\x12\x19.ari.stt.v1.ModelsRequest\x1a\x1a.ari.stt.v1.ModelsResponse\"\x00\x12S\n\x0cNLPFunctions\x12\x1f.ari.stt.v1.NLPFunctionsRequest\x1a .ari.stt.v1.NLPFunctionsResponse\"\x00\x12G\n\x07Locales\x12\x1a.ari.stt.v1.LocalesRequest\x1a\x1b.ari.stt.v1.LocalesResponse\"\x03\x88\x02\x01\x12P\n\x0b\x41\x63\x63ountInfo\x12\x1e.ari.stt.v1.AccountInfoRequest\x1a\x1f.ari.stt.v1.AccountInfoResponse\"\x00\x12M\n\nNLPProcess\x12\x1d.ari.stt.v1.NLPProcessRequest\x1a\x1e.ari.stt.v1.NLPProcessResponse\"\x00\x62\x06proto3')

_globals = globals()
_builder.BuildMessageAndEnumDescriptors(DESCRIPTOR, _globals)
_builder.BuildTopDescriptorsAndMessages(DESCRIPTOR, 'stt_service_pb2', _globals)
if not _descriptor._USE_C_DESCRIPTORS:
  DESCRIPTOR._loaded_options = None
  _globals['_STTSERVICE'].methods_by_name['Locales']._loaded_options = None
  _globals['_STTSERVICE'].methods_by_name['Locales']._serialized_options = b'\210\002\001'
  _globals['_ENDPOINTINGTYPE']._serialized_start=3367
  _globals['_ENDPOINTINGTYPE']._serialized_end=3401
  _globals['_MODELTYPE']._serialized_start=3403
  _globals['_MODELTYPE']._serialized_end=3479
  _globals['_GRAMMARTYPE']._serialized_start=3481
  _globals['_GRAMMARTYPE']._serialized_end=3540
  _globals['_STREAMINGRECOGNITIONREQUEST']._serialized_start=65
  _globals['_STREAMINGRECOGNITIONREQUEST']._serialized_end=189
  _globals['_STREAMINGRECOGNITIONRESPONSE']._serialized_start=192
  _globals['_STREAMINGRECOGNITIONRESPONSE']._serialized_end=342
  _globals['_RECOGNITIONCONFIG']._serialized_start=344
  _globals['_RECOGNITIONCONFIG']._serialized_end=415
  _globals['_RECOGNITIONSPEC']._serialized_start=418
  _globals['_RECOGNITIONSPEC']._serialized_end=885
  _globals['_RECOGNITIONSPEC_AUDIOENCODING']._serialized_start=796
  _globals['_RECOGNITIONSPEC_AUDIOENCODING']._serialized_end=861
  _globals['_NORMALIZATIONSPEC']._serialized_start=887
  _globals['_NORMALIZATIONSPEC']._serialized_end=997
  _globals['_NLPSPEC']._serialized_start=1000
  _globals['_NLPSPEC']._serialized_end=1254
  _globals['_NLPSPEC_NLPINPUTFIELD']._serialized_start=1177
  _globals['_NLPSPEC_NLPINPUTFIELD']._serialized_end=1254
  _globals['_NLPFUNCTIONSPEC']._serialized_start=1256
  _globals['_NLPFUNCTIONSPEC']._serialized_end=1299
  _globals['_ENDPOINTSPEC']._serialized_start=1302
  _globals['_ENDPOINTSPEC']._serialized_end=1490
  _globals['_VADSPEC']._serialized_start=1492
  _globals['_VADSPEC']._serialized_end=1566
  _globals['_SPEECHRECOGNITIONCHUNK']._serialized_start=1569
  _globals['_SPEECHRECOGNITIONCHUNK']._serialized_end=1698
  _globals['_SPEECHRECOGNITIONALTERNATIVE']._serialized_start=1701
  _globals['_SPEECHRECOGNITIONALTERNATIVE']._serialized_end=1886
  _globals['_WORDINFO']._serialized_start=1889
  _globals['_WORDINFO']._serialized_end=2114
  _globals['_PHONEINFO']._serialized_start=2116
  _globals['_PHONEINFO']._serialized_end=2234
  _globals['_MODELSREQUEST']._serialized_start=2236
  _globals['_MODELSREQUEST']._serialized_end=2251
  _globals['_MODELSRESPONSE']._serialized_start=2253
  _globals['_MODELSRESPONSE']._serialized_end=2303
  _globals['_MODEL']._serialized_start=2306
  _globals['_MODEL']._serialized_end=2594
  _globals['_NLPFUNCTIONSREQUEST']._serialized_start=2596
  _globals['_NLPFUNCTIONSREQUEST']._serialized_end=2617
  _globals['_NLPFUNCTIONSRESPONSE']._serialized_start=2619
  _globals['_NLPFUNCTIONSRESPONSE']._serialized_end=2688
  _globals['_NLPFUNCTIONSERVER']._serialized_start=2690
  _globals['_NLPFUNCTIONSERVER']._serialized_end=2776
  _globals['_NLPFUNCTION']._serialized_start=2778
  _globals['_NLPFUNCTION']._serialized_end=2838
  _globals['_LOCALESREQUEST']._serialized_start=2840
  _globals['_LOCALESREQUEST']._serialized_end=2856
  _globals['_LOCALESRESPONSE']._serialized_start=2858
  _globals['_LOCALESRESPONSE']._serialized_end=2911
  _globals['_LOCALE']._serialized_start=2913
  _globals['_LOCALE']._serialized_end=3023
  _globals['_GRAPH']._serialized_start=3025
  _globals['_GRAPH']._serialized_end=3065
  _globals['_ACCOUNTINFOREQUEST']._serialized_start=3067
  _globals['_ACCOUNTINFOREQUEST']._serialized_end=3087
  _globals['_ACCOUNTINFORESPONSE']._serialized_start=3090
  _globals['_ACCOUNTINFORESPONSE']._serialized_end=3260
  _globals['_NLPPROCESSREQUEST']._serialized_start=3262
  _globals['_NLPPROCESSREQUEST']._serialized_end=3329
  _globals['_NLPPROCESSRESPONSE']._serialized_start=3331
  _globals['_NLPPROCESSRESPONSE']._serialized_end=3365
  _globals['_STTSERVICE']._serialized_start=3543
  _globals['_STTSERVICE']._serialized_end=4052
# @@protoc_insertion_point(module_scope)
