"""
@generated by mypy-protobuf.  Do not edit manually!
isort:skip_file
"""

import builtins
import collections.abc
import google.protobuf.descriptor
import google.protobuf.duration_pb2
import google.protobuf.internal.containers
import google.protobuf.internal.enum_type_wrapper
import google.protobuf.message
import sys
import typing

if sys.version_info >= (3, 10):
    import typing as typing_extensions
else:
    import typing_extensions

DESCRIPTOR: google.protobuf.descriptor.FileDescriptor

class _EndpointingType:
    ValueType = typing.NewType("ValueType", builtins.int)
    V: typing_extensions.TypeAlias = ValueType

class _EndpointingTypeEnumTypeWrapper(google.protobuf.internal.enum_type_wrapper._EnumTypeWrapper[_EndpointingType.ValueType], builtins.type):
    DESCRIPTOR: google.protobuf.descriptor.EnumDescriptor
    LM: _EndpointingType.ValueType  # 0
    """Endpointing that considers the language model."""
    VAD: _EndpointingType.ValueType  # 1
    """VAD based endpointing."""

class EndpointingType(_EndpointingType, metaclass=_EndpointingTypeEnumTypeWrapper): ...

LM: EndpointingType.ValueType  # 0
"""Endpointing that considers the language model."""
VAD: EndpointingType.ValueType  # 1
"""VAD based endpointing."""
global___EndpointingType = EndpointingType

class _ModelType:
    ValueType = typing.NewType("ValueType", builtins.int)
    V: typing_extensions.TypeAlias = ValueType

class _ModelTypeEnumTypeWrapper(google.protobuf.internal.enum_type_wrapper._EnumTypeWrapper[_ModelType.ValueType], builtins.type):
    DESCRIPTOR: google.protobuf.descriptor.EnumDescriptor
    CORE_STT: _ModelType.ValueType  # 0
    """STT-Core models"""
    GRAMMAR_STT: _ModelType.ValueType  # 1
    """Grammar only models"""
    MULTITASK_STT: _ModelType.ValueType  # 2
    """Multitask models"""
    DIARIZATION: _ModelType.ValueType  # 3
    """Speaker diarization model type."""

class ModelType(_ModelType, metaclass=_ModelTypeEnumTypeWrapper): ...

CORE_STT: ModelType.ValueType  # 0
"""STT-Core models"""
GRAMMAR_STT: ModelType.ValueType  # 1
"""Grammar only models"""
MULTITASK_STT: ModelType.ValueType  # 2
"""Multitask models"""
DIARIZATION: ModelType.ValueType  # 3
"""Speaker diarization model type."""
global___ModelType = ModelType

class _GrammarType:
    ValueType = typing.NewType("ValueType", builtins.int)
    V: typing_extensions.TypeAlias = ValueType

class _GrammarTypeEnumTypeWrapper(google.protobuf.internal.enum_type_wrapper._EnumTypeWrapper[_GrammarType.ValueType], builtins.type):
    DESCRIPTOR: google.protobuf.descriptor.EnumDescriptor
    JSGF: _GrammarType.ValueType  # 0
    """JSGF grammar type.
    Example grammar: `jsgf:<yes_no> = yes | no;`
    """
    SRGS: _GrammarType.ValueType  # 3
    """SRGS grammar type.
    Example grammar: `srgs:$yes_no = yes | no;`
    """
    KWS: _GrammarType.ValueType  # 1
    """Keyword / Keyphrase spotting grammar type.
    Example grammar: `kws:oh mighty computer|hey computer`
    """
    PHRASE_LIST: _GrammarType.ValueType  # 2
    """A simple json phrase list grammar type.
    Example grammar: `["yes", "yeah", "yep", "why not", "no", "nope"]`
    """

class GrammarType(_GrammarType, metaclass=_GrammarTypeEnumTypeWrapper): ...

JSGF: GrammarType.ValueType  # 0
"""JSGF grammar type.
Example grammar: `jsgf:<yes_no> = yes | no;`
"""
SRGS: GrammarType.ValueType  # 3
"""SRGS grammar type.
Example grammar: `srgs:$yes_no = yes | no;`
"""
KWS: GrammarType.ValueType  # 1
"""Keyword / Keyphrase spotting grammar type.
Example grammar: `kws:oh mighty computer|hey computer`
"""
PHRASE_LIST: GrammarType.ValueType  # 2
"""A simple json phrase list grammar type.
Example grammar: `["yes", "yeah", "yep", "why not", "no", "nope"]`
"""
global___GrammarType = GrammarType

@typing.final
class StreamingRecognitionRequest(google.protobuf.message.Message):
    """The top-level message sent by the client for the `StreamingRecognize` method."""

    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    CONFIG_FIELD_NUMBER: builtins.int
    AUDIO_CONTENT_FIELD_NUMBER: builtins.int
    audio_content: builtins.bytes
    """The audio data to be recognized."""
    @property
    def config(self) -> global___RecognitionConfig:
        """The configuration for the stream.
        This is the first message that must be sent.
        """

    def __init__(
        self,
        *,
        config: global___RecognitionConfig | None = ...,
        audio_content: builtins.bytes = ...,
    ) -> None: ...
    def HasField(self, field_name: typing.Literal["audio_content", b"audio_content", "config", b"config", "streaming_request", b"streaming_request"]) -> builtins.bool: ...
    def ClearField(self, field_name: typing.Literal["audio_content", b"audio_content", "config", b"config", "streaming_request", b"streaming_request"]) -> None: ...
    def WhichOneof(self, oneof_group: typing.Literal["streaming_request", b"streaming_request"]) -> typing.Literal["config", "audio_content"] | None: ...

global___StreamingRecognitionRequest = StreamingRecognitionRequest

@typing.final
class StreamingRecognitionResponse(google.protobuf.message.Message):
    """The top-level message returned from the `StreamingRecognize` method."""

    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    CHUNKS_FIELD_NUMBER: builtins.int
    CLIENT_ID_FIELD_NUMBER: builtins.int
    LANGUAGE_FIELD_NUMBER: builtins.int
    client_id: builtins.str
    """A short id that is used in the stt-server logs to differentiate between
    different client requests. Be aware that this id should only be used for
    debugging purposes because it is not collision safe.
    """
    language: builtins.str
    """The language identified by the server, e.g. en, de, etc."""
    @property
    def chunks(self) -> google.protobuf.internal.containers.RepeatedCompositeFieldContainer[global___SpeechRecognitionChunk]:
        """List of results that are currently available."""

    def __init__(
        self,
        *,
        chunks: collections.abc.Iterable[global___SpeechRecognitionChunk] | None = ...,
        client_id: builtins.str = ...,
        language: builtins.str = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing.Literal["chunks", b"chunks", "client_id", b"client_id", "language", b"language"]) -> None: ...

global___StreamingRecognitionResponse = StreamingRecognitionResponse

@typing.final
class RecognitionConfig(google.protobuf.message.Message):
    """The `RecognitionConfig` message provides information to the recognizer that
    specifies how to process the request.
    """

    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    SPECIFICATION_FIELD_NUMBER: builtins.int
    @property
    def specification(self) -> global___RecognitionSpec:
        """Specifies what kind of audio is being sent and how the recognizer should
        process it.
        """

    def __init__(
        self,
        *,
        specification: global___RecognitionSpec | None = ...,
    ) -> None: ...
    def HasField(self, field_name: typing.Literal["specification", b"specification"]) -> builtins.bool: ...
    def ClearField(self, field_name: typing.Literal["specification", b"specification"]) -> None: ...

global___RecognitionConfig = RecognitionConfig

@typing.final
class RecognitionSpec(google.protobuf.message.Message):
    """The `RecognitionSpec` message provides information to the recognizer that
    specifies how to process the request.
    """

    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    class _AudioEncoding:
        ValueType = typing.NewType("ValueType", builtins.int)
        V: typing_extensions.TypeAlias = ValueType

    class _AudioEncodingEnumTypeWrapper(google.protobuf.internal.enum_type_wrapper._EnumTypeWrapper[RecognitionSpec._AudioEncoding.ValueType], builtins.type):
        DESCRIPTOR: google.protobuf.descriptor.EnumDescriptor
        AUDIO_ENCODING_UNSPECIFIED: RecognitionSpec._AudioEncoding.ValueType  # 0
        """If not specified, defaults to LINEAR16_PCM."""
        LINEAR16_PCM: RecognitionSpec._AudioEncoding.ValueType  # 1
        """16-bit signed little-endian (Linear PCM)"""

    class AudioEncoding(_AudioEncoding, metaclass=_AudioEncodingEnumTypeWrapper): ...
    AUDIO_ENCODING_UNSPECIFIED: RecognitionSpec.AudioEncoding.ValueType  # 0
    """If not specified, defaults to LINEAR16_PCM."""
    LINEAR16_PCM: RecognitionSpec.AudioEncoding.ValueType  # 1
    """16-bit signed little-endian (Linear PCM)"""

    AUDIO_ENCODING_FIELD_NUMBER: builtins.int
    SAMPLE_RATE_HERTZ_FIELD_NUMBER: builtins.int
    LOCALE_FIELD_NUMBER: builtins.int
    GRAPH_FIELD_NUMBER: builtins.int
    GRAMMAR_FIELD_NUMBER: builtins.int
    PARTIAL_RESULTS_FIELD_NUMBER: builtins.int
    SINGLE_UTTERANCE_FIELD_NUMBER: builtins.int
    NORMALIZATION_FIELD_NUMBER: builtins.int
    PHONES_FIELD_NUMBER: builtins.int
    MODEL_FIELD_NUMBER: builtins.int
    ENDPOINTING_FIELD_NUMBER: builtins.int
    VAD_FIELD_NUMBER: builtins.int
    PROMPT_FIELD_NUMBER: builtins.int
    audio_encoding: global___RecognitionSpec.AudioEncoding.ValueType
    """At the moment only LINEAR16 is supported."""
    sample_rate_hertz: builtins.int
    """8000, 16000, 48000 only for pcm."""
    locale: builtins.str
    """[language[_territory]] e.g. en, en-IN, de."""
    graph: builtins.str
    """load a specific graph for the locale specific model (e.g. yes_no)."""
    grammar: builtins.str
    """Allows to specify a grammar to be used for the recognition.
    To specify a JSGF grammar for example set grammar to `jsgf:public <yes_no>
    = yes | no;` To spot a keyword / phrase, set grammar to `kws:oh mighty
    computer` You can also specify a json string to narrow the possible words
    to appear `["oh one two three four five six seven eight nine zero",
    "[unk]"]`.
    """
    partial_results: builtins.bool
    """If set true, tentative hypotheses may be returned as they become available
    (final=false flag) If false or omitted, only final=true result(s) are
    returned. Makes sense only for StreamingRecognize requests.
    """
    single_utterance: builtins.bool
    """Decode as single utterance."""
    phones: builtins.bool
    """When set, the recognizer opts out of MBR decoding and produces phoneme
    infos.
    """
    model: builtins.str
    """Instead of picking a model based on the locale, this field can be used to
    specify a specific model directly.
    To specify a graph model directly use model:graph e.g.
    generic-model-de-0.21:ja_nein
    """
    prompt: builtins.str
    """Some models allow to specify a prompt that can be used to give the model
    some context on what was said before or to steer the model to use
    particular spellings or styles.
    """
    @property
    def normalization(self) -> global___NormalizationSpec:
        """Specifies how text should be normalized."""

    @property
    def endpointing(self) -> global___EndpointSpec:
        """For models that use endpointing (e.g. STT-Core models) this field can be
        used to specify the endpointing configuration.
        """

    @property
    def vad(self) -> global___VadSpec:
        """For models that use voice activity detection (VAD) this field can be used
        to specify the VAD configuration.
        """

    def __init__(
        self,
        *,
        audio_encoding: global___RecognitionSpec.AudioEncoding.ValueType = ...,
        sample_rate_hertz: builtins.int = ...,
        locale: builtins.str = ...,
        graph: builtins.str = ...,
        grammar: builtins.str = ...,
        partial_results: builtins.bool = ...,
        single_utterance: builtins.bool = ...,
        normalization: global___NormalizationSpec | None = ...,
        phones: builtins.bool = ...,
        model: builtins.str = ...,
        endpointing: global___EndpointSpec | None = ...,
        vad: global___VadSpec | None = ...,
        prompt: builtins.str = ...,
    ) -> None: ...
    def HasField(self, field_name: typing.Literal["endpointing", b"endpointing", "normalization", b"normalization", "vad", b"vad"]) -> builtins.bool: ...
    def ClearField(self, field_name: typing.Literal["audio_encoding", b"audio_encoding", "endpointing", b"endpointing", "grammar", b"grammar", "graph", b"graph", "locale", b"locale", "model", b"model", "normalization", b"normalization", "partial_results", b"partial_results", "phones", b"phones", "prompt", b"prompt", "sample_rate_hertz", b"sample_rate_hertz", "single_utterance", b"single_utterance", "vad", b"vad"]) -> None: ...

global___RecognitionSpec = RecognitionSpec

@typing.final
class NormalizationSpec(google.protobuf.message.Message):
    """The `NormalizationSpec` message provides information to the recognizer that
    specifies which normalizer to use.
    """

    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    STRIP_UNK_FIELD_NUMBER: builtins.int
    NLP_FIELD_NUMBER: builtins.int
    strip_unk: builtins.bool
    """Whether to strip unknown label in the resulting text
    Note: The unknown label can still be accessed via the `words` field
    """
    @property
    def nlp(self) -> global___NLPSpec:
        """Allows to specify which nlp functions should be applied to the text
        before it is returned. By specifying this field, the default nlp
        configuration is overwritten.
        """

    def __init__(
        self,
        *,
        strip_unk: builtins.bool = ...,
        nlp: global___NLPSpec | None = ...,
    ) -> None: ...
    def HasField(self, field_name: typing.Literal["nlp", b"nlp"]) -> builtins.bool: ...
    def ClearField(self, field_name: typing.Literal["nlp", b"nlp", "strip_unk", b"strip_unk"]) -> None: ...

global___NormalizationSpec = NormalizationSpec

@typing.final
class NLPSpec(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    class _NlpInputField:
        ValueType = typing.NewType("ValueType", builtins.int)
        V: typing_extensions.TypeAlias = ValueType

    class _NlpInputFieldEnumTypeWrapper(google.protobuf.internal.enum_type_wrapper._EnumTypeWrapper[NLPSpec._NlpInputField.ValueType], builtins.type):
        DESCRIPTOR: google.protobuf.descriptor.EnumDescriptor
        UNSPECIFIED: NLPSpec._NlpInputField.ValueType  # 0
        """If not specified, defaults to TEXT."""
        TEXT: NLPSpec._NlpInputField.ValueType  # 1
        """The text field is used as input for the nlp processing."""
        TAGGED_TEXT: NLPSpec._NlpInputField.ValueType  # 2
        """Use the tagged_text field as input for the nlp processing."""
        SLOTTED_TEXT: NLPSpec._NlpInputField.ValueType  # 3
        """Use the slotted_text field as input for the nlp processing."""

    class NlpInputField(_NlpInputField, metaclass=_NlpInputFieldEnumTypeWrapper): ...
    UNSPECIFIED: NLPSpec.NlpInputField.ValueType  # 0
    """If not specified, defaults to TEXT."""
    TEXT: NLPSpec.NlpInputField.ValueType  # 1
    """The text field is used as input for the nlp processing."""
    TAGGED_TEXT: NLPSpec.NlpInputField.ValueType  # 2
    """Use the tagged_text field as input for the nlp processing."""
    SLOTTED_TEXT: NLPSpec.NlpInputField.ValueType  # 3
    """Use the slotted_text field as input for the nlp processing."""

    SERVER_CONFIG_FIELD_NUMBER: builtins.int
    FUNCTIONS_FIELD_NUMBER: builtins.int
    PARTIAL_RESULTS_FIELD_NUMBER: builtins.int
    ARGS_FIELD_NUMBER: builtins.int
    INPUT_FIELD_FIELD_NUMBER: builtins.int
    server_config: builtins.str
    """The server config name of the server that provides the nlp functions."""
    partial_results: builtins.bool
    """Whether to apply the nlp functions to the partial results."""
    args: builtins.str
    """Optional global argument."""
    input_field: global___NLPSpec.NlpInputField.ValueType
    """Specifies which field should be used as input for the nlp functions."""
    @property
    def functions(self) -> google.protobuf.internal.containers.RepeatedCompositeFieldContainer[global___NLPFunctionSpec]:
        """Which nlp functions should be applied to the text before it is returned."""

    def __init__(
        self,
        *,
        server_config: builtins.str = ...,
        functions: collections.abc.Iterable[global___NLPFunctionSpec] | None = ...,
        partial_results: builtins.bool = ...,
        args: builtins.str = ...,
        input_field: global___NLPSpec.NlpInputField.ValueType = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing.Literal["args", b"args", "functions", b"functions", "input_field", b"input_field", "partial_results", b"partial_results", "server_config", b"server_config"]) -> None: ...

global___NLPSpec = NLPSpec

@typing.final
class NLPFunctionSpec(google.protobuf.message.Message):
    """The `NLPFunction` message provides information to the recognizer that
    specifies which nlp function to use.
    """

    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    ID_FIELD_NUMBER: builtins.int
    ARGS_FIELD_NUMBER: builtins.int
    id: builtins.str
    """The id of the nlp function (e.g. `ner-de`)."""
    @property
    def args(self) -> google.protobuf.internal.containers.RepeatedScalarFieldContainer[builtins.str]:
        """Optional additional parameters for the nlp function (e.g. `ANONYMIZE`)."""

    def __init__(
        self,
        *,
        id: builtins.str = ...,
        args: collections.abc.Iterable[builtins.str] | None = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing.Literal["args", b"args", "id", b"id"]) -> None: ...

global___NLPFunctionSpec = NLPFunctionSpec

@typing.final
class EndpointSpec(google.protobuf.message.Message):
    """Endpointing configuration for LM based endpointing."""

    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    SILENCE_TIMEOUT_FIELD_NUMBER: builtins.int
    TRAILING_SILENCE_HIGH_PROBABILITY_FIELD_NUMBER: builtins.int
    TRAILING_SILENCE_OK_PROBABILITY_FIELD_NUMBER: builtins.int
    TRAILING_SILENCE_NO_ENDPOINT_FIELD_NUMBER: builtins.int
    UTTERANCE_TIMEOUT_FIELD_NUMBER: builtins.int
    silence_timeout: builtins.float
    """How many seconds of non-speech before the endpointer triggers to
    clean up the buffer.
    """
    trailing_silence_high_probability: builtins.float
    """How many seconds of non-speech after some speech with high
    probability for an endpoint before the endpointer triggers.
    """
    trailing_silence_ok_probability: builtins.float
    """How many seconds of non-speech after some speech with low probability
    for an endpoint before the endpointer triggers.
    """
    trailing_silence_no_endpoint: builtins.float
    """How many seconds of non-speech without the endpointer reaching a
    final state before the endpointer triggers.
    """
    utterance_timeout: builtins.float
    """After how many seconds of audio to trigger and endpoint regardless
    of anything else.
    """
    def __init__(
        self,
        *,
        silence_timeout: builtins.float = ...,
        trailing_silence_high_probability: builtins.float = ...,
        trailing_silence_ok_probability: builtins.float = ...,
        trailing_silence_no_endpoint: builtins.float = ...,
        utterance_timeout: builtins.float = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing.Literal["silence_timeout", b"silence_timeout", "trailing_silence_high_probability", b"trailing_silence_high_probability", "trailing_silence_no_endpoint", b"trailing_silence_no_endpoint", "trailing_silence_ok_probability", b"trailing_silence_ok_probability", "utterance_timeout", b"utterance_timeout"]) -> None: ...

global___EndpointSpec = EndpointSpec

@typing.final
class VadSpec(google.protobuf.message.Message):
    """Endpointing configuration for Voice activity detection (VAD) based
    endpointing.
    """

    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    THRESHOLD_FIELD_NUMBER: builtins.int
    TRAILING_SILENCE_FIELD_NUMBER: builtins.int
    MIN_SPEECH_FIELD_NUMBER: builtins.int
    threshold: builtins.float
    """The threshold between 0 and 1.0 to determine if a frame is speech or
    non-speech. A higher threshold will result in less false positives but also
    some speech might be cut off.
    """
    trailing_silence: builtins.float
    """Amount of trailing silence before an utterance is considered after a speech
    to non-speech transition.
    """
    min_speech: builtins.float
    """The minimum duration of speech in seconds before trying to perform a
    partial recognition.
    """
    def __init__(
        self,
        *,
        threshold: builtins.float = ...,
        trailing_silence: builtins.float = ...,
        min_speech: builtins.float = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing.Literal["min_speech", b"min_speech", "threshold", b"threshold", "trailing_silence", b"trailing_silence"]) -> None: ...

global___VadSpec = VadSpec

@typing.final
class SpeechRecognitionChunk(google.protobuf.message.Message):
    """The `SpeechRecognitionChunk` message contains the result of a single
    utterance.
    """

    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    ALTERNATIVES_FIELD_NUMBER: builtins.int
    FINAL_FIELD_NUMBER: builtins.int
    END_OF_UTTERANCE_FIELD_NUMBER: builtins.int
    final: builtins.bool
    """This flag indicates if the transcription is final or not."""
    end_of_utterance: builtins.bool
    """This flag shows that the received chunk is the end of an utterance."""
    @property
    def alternatives(self) -> google.protobuf.internal.containers.RepeatedCompositeFieldContainer[global___SpeechRecognitionAlternative]:
        """The transcription alternatives."""

    def __init__(
        self,
        *,
        alternatives: collections.abc.Iterable[global___SpeechRecognitionAlternative] | None = ...,
        final: builtins.bool = ...,
        end_of_utterance: builtins.bool = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing.Literal["alternatives", b"alternatives", "end_of_utterance", b"end_of_utterance", "final", b"final"]) -> None: ...

global___SpeechRecognitionChunk = SpeechRecognitionChunk

@typing.final
class SpeechRecognitionAlternative(google.protobuf.message.Message):
    """The `SpeechRecognitionAlternative` message contains one alternative of a
    transcription.
    """

    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    TEXT_FIELD_NUMBER: builtins.int
    SLOTTED_TEXT_FIELD_NUMBER: builtins.int
    TAGGED_TEXT_FIELD_NUMBER: builtins.int
    NLP_TEXT_FIELD_NUMBER: builtins.int
    CONFIDENCE_FIELD_NUMBER: builtins.int
    WORDS_FIELD_NUMBER: builtins.int
    text: builtins.str
    """The raw recognized text."""
    slotted_text: builtins.str
    """When the model is composed of multiple nested language models, this field
    contains the recognized text including xml tags that indicate which
    language model produced which part of the text. e.g. "i live in <address>
    <number> 21 </number> <street> jumpstreet </street> <city> heidelberg
    </city> </address>"
    """
    tagged_text: builtins.str
    """The tagged recognized text."""
    nlp_text: builtins.str
    """The nlp result."""
    confidence: builtins.float
    """The overall confidence of the recognition result."""
    @property
    def words(self) -> google.protobuf.internal.containers.RepeatedCompositeFieldContainer[global___WordInfo]:
        """Word level infos such as start and end time offsets, word level
        confidences, or phoneme infos.
        """

    def __init__(
        self,
        *,
        text: builtins.str = ...,
        slotted_text: builtins.str = ...,
        tagged_text: builtins.str = ...,
        nlp_text: builtins.str = ...,
        confidence: builtins.float = ...,
        words: collections.abc.Iterable[global___WordInfo] | None = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing.Literal["confidence", b"confidence", "nlp_text", b"nlp_text", "slotted_text", b"slotted_text", "tagged_text", b"tagged_text", "text", b"text", "words", b"words"]) -> None: ...

global___SpeechRecognitionAlternative = SpeechRecognitionAlternative

@typing.final
class WordInfo(google.protobuf.message.Message):
    """The `WordInfo` message contains the word level information."""

    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    START_TIME_FIELD_NUMBER: builtins.int
    END_TIME_FIELD_NUMBER: builtins.int
    WORD_FIELD_NUMBER: builtins.int
    CONFIDENCE_FIELD_NUMBER: builtins.int
    PHONES_FIELD_NUMBER: builtins.int
    SLOT_FIELD_NUMBER: builtins.int
    word: builtins.str
    """The word."""
    confidence: builtins.float
    """The confidence of the word in the range [0.0, 1.0]."""
    slot: builtins.str
    """Speech recognition slot the word belongs to.
    For nested slots, the slots are joined with a dot and ordered from outer to
    inner
    e.g.: "i live in <address> <number> 21 </number> <street> jumpstreet
    </street> heidelberg </address>"
    will have the following slots:
    i -> ''
    live -> ''
    in -> ''
    21 -> 'address.number'
    jumpstreet -> 'address.street'
    heidelberg -> 'address'
    """
    @property
    def start_time(self) -> google.protobuf.duration_pb2.Duration:
        """The word's start time, in seconds."""

    @property
    def end_time(self) -> google.protobuf.duration_pb2.Duration:
        """The word's end time, in seconds."""

    @property
    def phones(self) -> google.protobuf.internal.containers.RepeatedCompositeFieldContainer[global___PhoneInfo]:
        """Phoneme infos."""

    def __init__(
        self,
        *,
        start_time: google.protobuf.duration_pb2.Duration | None = ...,
        end_time: google.protobuf.duration_pb2.Duration | None = ...,
        word: builtins.str = ...,
        confidence: builtins.float = ...,
        phones: collections.abc.Iterable[global___PhoneInfo] | None = ...,
        slot: builtins.str = ...,
    ) -> None: ...
    def HasField(self, field_name: typing.Literal["end_time", b"end_time", "start_time", b"start_time"]) -> builtins.bool: ...
    def ClearField(self, field_name: typing.Literal["confidence", b"confidence", "end_time", b"end_time", "phones", b"phones", "slot", b"slot", "start_time", b"start_time", "word", b"word"]) -> None: ...

global___WordInfo = WordInfo

@typing.final
class PhoneInfo(google.protobuf.message.Message):
    """The `PhoneInfo` message contains the phoneme level information."""

    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    START_TIME_FIELD_NUMBER: builtins.int
    END_TIME_FIELD_NUMBER: builtins.int
    PHONE_FIELD_NUMBER: builtins.int
    phone: builtins.str
    """The phone."""
    @property
    def start_time(self) -> google.protobuf.duration_pb2.Duration:
        """The phone's start time, in seconds."""

    @property
    def end_time(self) -> google.protobuf.duration_pb2.Duration:
        """The phone's end time, in seconds."""

    def __init__(
        self,
        *,
        start_time: google.protobuf.duration_pb2.Duration | None = ...,
        end_time: google.protobuf.duration_pb2.Duration | None = ...,
        phone: builtins.str = ...,
    ) -> None: ...
    def HasField(self, field_name: typing.Literal["end_time", b"end_time", "start_time", b"start_time"]) -> builtins.bool: ...
    def ClearField(self, field_name: typing.Literal["end_time", b"end_time", "phone", b"phone", "start_time", b"start_time"]) -> None: ...

global___PhoneInfo = PhoneInfo

@typing.final
class ModelsRequest(google.protobuf.message.Message):
    """The `ModelsRequest` message currently contains no information."""

    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    def __init__(
        self,
    ) -> None: ...

global___ModelsRequest = ModelsRequest

@typing.final
class ModelsResponse(google.protobuf.message.Message):
    """The `ModelsResponse` message contains the list of supported models."""

    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    MODEL_FIELD_NUMBER: builtins.int
    @property
    def model(self) -> google.protobuf.internal.containers.RepeatedCompositeFieldContainer[global___Model]:
        """List of supported models."""

    def __init__(
        self,
        *,
        model: collections.abc.Iterable[global___Model] | None = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing.Literal["model", b"model"]) -> None: ...

global___ModelsResponse = ModelsResponse

@typing.final
class Model(google.protobuf.message.Message):
    """The `Model` message contains the information about a single model."""

    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    ID_FIELD_NUMBER: builtins.int
    ALIAS_FIELD_NUMBER: builtins.int
    NAME_FIELD_NUMBER: builtins.int
    DESCRIPTION_FIELD_NUMBER: builtins.int
    VERSION_FIELD_NUMBER: builtins.int
    TYPE_FIELD_NUMBER: builtins.int
    LOCALE_FIELD_NUMBER: builtins.int
    GRAMMAR_TYPE_FIELD_NUMBER: builtins.int
    NLP_FIELD_NUMBER: builtins.int
    SLOTS_FIELD_NUMBER: builtins.int
    EXAMPLES_FIELD_NUMBER: builtins.int
    ENDPOINTING_FIELD_NUMBER: builtins.int
    id: builtins.str
    """The model id.
    e.g. generic-model-de-0.21
    """
    name: builtins.str
    """The human readable model name (for display purposes).
    e.g. German Generic Model (Large)
    """
    description: builtins.str
    """The model description."""
    version: builtins.str
    """The model version."""
    type: global___ModelType.ValueType
    """The model type."""
    @property
    def alias(self) -> google.protobuf.internal.containers.RepeatedScalarFieldContainer[builtins.str]:
        """Alias that can also be used to refer to the model instead of the id.
        e.g. generic-de or german-large
        """

    @property
    def locale(self) -> google.protobuf.internal.containers.RepeatedScalarFieldContainer[builtins.str]:
        """The locale(s) supported by the model."""

    @property
    def grammar_type(self) -> google.protobuf.internal.containers.RepeatedScalarFieldContainer[global___GrammarType.ValueType]:
        """Which grammar types are supported by the model."""

    @property
    def nlp(self) -> global___NLPSpec:
        """The NLP preconfiguration for this model (if any)."""

    @property
    def slots(self) -> google.protobuf.internal.containers.RepeatedScalarFieldContainer[builtins.str]:
        """The slots the model potentially outputs."""

    @property
    def examples(self) -> google.protobuf.internal.containers.RepeatedScalarFieldContainer[builtins.str]:
        """Examples of what the model can recognize."""

    @property
    def endpointing(self) -> google.protobuf.internal.containers.RepeatedScalarFieldContainer[global___EndpointingType.ValueType]:
        """The supported endpointing modes."""

    def __init__(
        self,
        *,
        id: builtins.str = ...,
        alias: collections.abc.Iterable[builtins.str] | None = ...,
        name: builtins.str = ...,
        description: builtins.str = ...,
        version: builtins.str = ...,
        type: global___ModelType.ValueType = ...,
        locale: collections.abc.Iterable[builtins.str] | None = ...,
        grammar_type: collections.abc.Iterable[global___GrammarType.ValueType] | None = ...,
        nlp: global___NLPSpec | None = ...,
        slots: collections.abc.Iterable[builtins.str] | None = ...,
        examples: collections.abc.Iterable[builtins.str] | None = ...,
        endpointing: collections.abc.Iterable[global___EndpointingType.ValueType] | None = ...,
    ) -> None: ...
    def HasField(self, field_name: typing.Literal["nlp", b"nlp"]) -> builtins.bool: ...
    def ClearField(self, field_name: typing.Literal["alias", b"alias", "description", b"description", "endpointing", b"endpointing", "examples", b"examples", "grammar_type", b"grammar_type", "id", b"id", "locale", b"locale", "name", b"name", "nlp", b"nlp", "slots", b"slots", "type", b"type", "version", b"version"]) -> None: ...

global___Model = Model

@typing.final
class NLPFunctionsRequest(google.protobuf.message.Message):
    """The `NLPFunctionsRequest` message currently contains no information."""

    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    def __init__(
        self,
    ) -> None: ...

global___NLPFunctionsRequest = NLPFunctionsRequest

@typing.final
class NLPFunctionsResponse(google.protobuf.message.Message):
    """The `NLPFunctionsResponse` message contains the list of supported nlp
    servers and the corresponding functions.
    """

    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    SERVER_FIELD_NUMBER: builtins.int
    @property
    def server(self) -> google.protobuf.internal.containers.RepeatedCompositeFieldContainer[global___NLPFunctionServer]:
        """List of supported nlp servers."""

    def __init__(
        self,
        *,
        server: collections.abc.Iterable[global___NLPFunctionServer] | None = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing.Literal["server", b"server"]) -> None: ...

global___NLPFunctionsResponse = NLPFunctionsResponse

@typing.final
class NLPFunctionServer(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    SERVER_CONFIG_FIELD_NUMBER: builtins.int
    FUNCTIONS_FIELD_NUMBER: builtins.int
    server_config: builtins.str
    """The nlp server configuration name (to be used in `NLPSpec`)."""
    @property
    def functions(self) -> google.protobuf.internal.containers.RepeatedCompositeFieldContainer[global___NLPFunction]:
        """The nlp functions supported by the nlp server."""

    def __init__(
        self,
        *,
        server_config: builtins.str = ...,
        functions: collections.abc.Iterable[global___NLPFunction] | None = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing.Literal["functions", b"functions", "server_config", b"server_config"]) -> None: ...

global___NLPFunctionServer = NLPFunctionServer

@typing.final
class NLPFunction(google.protobuf.message.Message):
    """The `NLPFunction` message contains the information about a single nlp
    function.
    """

    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    ID_FIELD_NUMBER: builtins.int
    NAME_FIELD_NUMBER: builtins.int
    DESCRIPTION_FIELD_NUMBER: builtins.int
    id: builtins.str
    """The nlp function id."""
    name: builtins.str
    """The nlp function name."""
    description: builtins.str
    """The nlp function description."""
    def __init__(
        self,
        *,
        id: builtins.str = ...,
        name: builtins.str = ...,
        description: builtins.str = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing.Literal["description", b"description", "id", b"id", "name", b"name"]) -> None: ...

global___NLPFunction = NLPFunction

@typing.final
class Graph(google.protobuf.message.Message):
    """The `Graph` message contains the information about a single graph."""

    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    NAME_FIELD_NUMBER: builtins.int
    name: builtins.str
    """The name of the graph"""
    def __init__(
        self,
        *,
        name: builtins.str = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing.Literal["name", b"name"]) -> None: ...

global___Graph = Graph

@typing.final
class AccountInfoRequest(google.protobuf.message.Message):
    """The `AccountInfoRequest` message currently contains no information."""

    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    def __init__(
        self,
    ) -> None: ...

global___AccountInfoRequest = AccountInfoRequest

@typing.final
class AccountInfoResponse(google.protobuf.message.Message):
    """The `AccountInfoResponse` message contains the account information."""

    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    TOKEN_FIELD_NUMBER: builtins.int
    DISPLAY_NAME_FIELD_NUMBER: builtins.int
    TOTAL_REQUESTS_FIELD_NUMBER: builtins.int
    BOOKED_SECONDS_FIELD_NUMBER: builtins.int
    USED_SECONDS_FIELD_NUMBER: builtins.int
    EXPIRATION_DATE_FIELD_NUMBER: builtins.int
    BLOCKED_FIELD_NUMBER: builtins.int
    token: builtins.str
    """The account token."""
    display_name: builtins.str
    """The account display name."""
    total_requests: builtins.int
    """How many requests were made with this account."""
    booked_seconds: builtins.int
    """How many seconds of audio this account has booked."""
    used_seconds: builtins.int
    """How many seconds of audio this account has used."""
    expiration_date: builtins.int
    """Expiration date of the account as unix timestamp (-1 for unlimited)."""
    blocked: builtins.bool
    """Whether the account is blocked."""
    def __init__(
        self,
        *,
        token: builtins.str = ...,
        display_name: builtins.str = ...,
        total_requests: builtins.int = ...,
        booked_seconds: builtins.int = ...,
        used_seconds: builtins.int = ...,
        expiration_date: builtins.int = ...,
        blocked: builtins.bool = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing.Literal["blocked", b"blocked", "booked_seconds", b"booked_seconds", "display_name", b"display_name", "expiration_date", b"expiration_date", "token", b"token", "total_requests", b"total_requests", "used_seconds", b"used_seconds"]) -> None: ...

global___AccountInfoResponse = AccountInfoResponse

@typing.final
class NLPProcessRequest(google.protobuf.message.Message):
    """The `NLPProcessRequest` message contains the text to be processed."""

    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    TEXT_FIELD_NUMBER: builtins.int
    NLP_FIELD_NUMBER: builtins.int
    text: builtins.str
    """The text to be processed."""
    @property
    def nlp(self) -> global___NLPSpec:
        """The nlp specification."""

    def __init__(
        self,
        *,
        text: builtins.str = ...,
        nlp: global___NLPSpec | None = ...,
    ) -> None: ...
    def HasField(self, field_name: typing.Literal["nlp", b"nlp"]) -> builtins.bool: ...
    def ClearField(self, field_name: typing.Literal["nlp", b"nlp", "text", b"text"]) -> None: ...

global___NLPProcessRequest = NLPProcessRequest

@typing.final
class NLPProcessResponse(google.protobuf.message.Message):
    """The `NLPProcessResponse` message contains the processed text."""

    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    TEXT_FIELD_NUMBER: builtins.int
    text: builtins.str
    """The processed text."""
    def __init__(
        self,
        *,
        text: builtins.str = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing.Literal["text", b"text"]) -> None: ...

global___NLPProcessResponse = NLPProcessResponse
