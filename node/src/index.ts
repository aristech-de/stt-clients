import * as grpc from '@grpc/grpc-js'

import {
  SttServiceClient,
  AccountInfoRequest,
  type AccountInfoResponse,
  ModelsRequest,
  NLPFunctionsRequest,
  type NLPFunctionsResponse,
  NLPProcessRequest,
  type NLPProcessResponse,
  RecognitionConfig,
  StreamingRecognitionRequest,
  Model,
  StreamingRecognitionResponse,
  DeepPartial,
} from './generated/stt_service.js'

import fs from 'fs'

// Re-export generated types
export * from './generated/stt_service.js'

export interface ConnectionOptions {
  /**
   * The Aristech STT-Server uri e.g. stt.example.com
   */
  host?: string
  /**
   * Whether to use SSL/TLS. Automatically enabled when rootCert is provided
   */
  ssl?: boolean
  /**
   * Allows providing a custom root certificate that might not exist
   * in the root certificate chain
   */
  rootCert?: string
  /**
   * Optionally instead of providing a root cert path via `rootCert` the root cert content can be provided directly
   */
  rootCertContent?: string
  /**
   * Further grpc client options
   */
  grpcClientOptions?: grpc.ClientOptions
  /**
   * Authentication options.
   * **Note:** Can only be used in combination with SSL/TLS.
   */
  auth?: {
    token: string
    secret: string
  }
}

export class SttClient {
  private cOptions: ConnectionOptions

  constructor(options: ConnectionOptions) {
    this.cOptions = options
  }

  /**
   * Lists the available models and their specifications.
   */
  listModels(request?: DeepPartial<ModelsRequest>): Promise<Model[]> {
    return new Promise((res, rej) => {
      const client = this.getClient()
      const req = ModelsRequest.create(request)
      client.models(req, (error, response) => {
        if (error) {
          rej(error)
          return
        }
        res(response.model)
      })
    })
  }

  /**
   * Creates a bidirectional recognition stream.
   * @param config The recognition configuration.
   * @returns The recognition stream.
   */
  recognize(config: DeepPartial<RecognitionConfig>) {
    const client = this.getClient()
    const call = client.streamingRecognize()
    const request = StreamingRecognitionRequest.create({ config })
    call.write(request)
    return call
  }


  /**
   * Recognizes a wave file.
   * This is a convenience method to very easily recognize a wave file.
   * @param waveFilePath Path to the wave file.
   * @param config The recognition configuration. The sample rate is automatically determined from the wave file. If you don't provide a config, only the locale will be set to 'en' so that the server can determine which model to use. We usually recomment to provide a specific model however.
   * @returns The recognition response.
   */
  recognizeFile(waveFilePath: string, config?: DeepPartial<RecognitionConfig>) {
    const client = this.getClient()
    const call = client.streamingRecognize()
    const sampleRate = config?.specification?.sampleRateHertz || getWaveSampleRate(waveFilePath)
    const request = StreamingRecognitionRequest.create({
      config: {
        ...config,
        specification: {
          locale: 'en',
          ...config?.specification,
          sampleRateHertz: sampleRate,
          partialResults: false,
        }
    } })
    call.write(request)
    const stream = fs.createReadStream(waveFilePath)
    stream.on('data', (chunk: Buffer) => {
      const audioContent = Uint8Array.from(chunk)
      const request = StreamingRecognitionRequest.create({ audioContent })
      call.write(request)
    })
    stream.on('end', () => {
      call.end()
    })
    return new Promise<StreamingRecognitionResponse[]>((res, rej) => {
      const result: StreamingRecognitionResponse[] = []
      call.on('data', (response: StreamingRecognitionResponse) => {
        result.push(response)
      })
      call.on('error', (error) => {
        rej(error)
      })
      call.on('end', () => {
        res(result)
      })
    })
  }

  /**
   * Lists the available NLP functions and their specifications.
   * @param request The NLP functions request.
   * @returns The NLP functions response.
   */
  listNlpFunctions(request?: DeepPartial<NLPFunctionsRequest>): Promise<NLPFunctionsResponse> {
    return new Promise((res, rej) => {
      const client = this.getClient()
      const req = NLPFunctionsRequest.create(request)
      client.nlpFunctions(req, (error, response) => {
        if (error) {
          rej(error)
          return
        }
        res(response)
      })
    })
  }

  /**
   * Performs NLP processing on the given text.
   * @param request The NLP processing request.
   * @returns The NLP processing response.
   */
  nlpProcess(request: DeepPartial<NLPProcessRequest>): Promise<NLPProcessResponse> {
    return new Promise((res, rej) => {
      const client = this.getClient()
      const req = NLPProcessRequest.create(request)
      client.nlpProcess(req, (error, response) => {
        if (error) {
          rej(error)
          return
        }
        res(response)
      })
    })
  }

  /**
   * Retrieves the account information.
   * @param request The account info request.
   * @returns The account info response.
   */
  accountInfo(request: DeepPartial<AccountInfoRequest>): Promise<AccountInfoResponse> {
    return new Promise((res, rej) => {
      const client = this.getClient()
      const req = AccountInfoRequest.create(request)
      client.accountInfo(req, (error, response) => {
        if (error) {
          rej(error)
          return
        }
        res(response)
      })
    })
  }

  private getClient() {
    const { rootCert: rootCertPath, rootCertContent, auth, grpcClientOptions } = this.cOptions
    let host = this.cOptions.host || 'localhost:9423'
    let ssl = this.cOptions.ssl === true
    let rootCert: Buffer | null = null
    if (rootCertContent) {
      rootCert = Buffer.from(rootCertContent)
    } else if (rootCertPath) {
      rootCert = fs.readFileSync(rootCertPath)
    }
    const sslExplicit = typeof this.cOptions.ssl === 'boolean' || !!rootCert
    const portRe = /[^:]+:([0-9]+)$/
    if (portRe.test(host)) {
      // In case a port was provided but ssl was not specified
      // ssl is assumed when the port matches 9424
      const [, portStr] = host.match(portRe)!
      const hostPort = parseInt(portStr, 10)
      if (!sslExplicit) {
        if (hostPort === 9424) {
          ssl = true
        } else {
          ssl = false
        }
      }
    } else {
      // In case no port was provided, depending on the ssl settings
      // at the default non ssl port 9423 or ssl port 9424
      if (sslExplicit && ssl) {
        host = `${host}:9424`
      } else {
        host = `${host}:9423`
      }
    }

    let creds = grpc.credentials.createInsecure()
    if (ssl || rootCert) {
      creds = grpc.credentials.createSsl(rootCert)
      if (auth) {
        const callCreds = grpc.credentials.createFromMetadataGenerator(
          (_, cb) => {
            const meta = new grpc.Metadata()
            meta.add('token', auth.token)
            meta.add('secret', auth.secret)
            cb(null, meta)
          },
        )
        creds = grpc.credentials.combineChannelCredentials(creds, callCreds)
      }
    }
    return new SttServiceClient(host, creds, grpcClientOptions)
  }
}

/**
 * A very simple helper function that reads the sample rate from a wave file (assuming it is a valid wave file with a 44 byte header).
 * @param fileName The path to the wave file.
 * @returns The sample rate of the wave file in Hz.
 */
export function getWaveSampleRate(fileName: string) {
  // Read the wave header to get the sample rate
  const header = Buffer.alloc(44)
  fs.readSync(fs.openSync(fileName, 'r'), header, 0, 44, 0)
  // The sample rate is stored in bytes 24-27 of the wave header
  return header.readUInt32LE(24)
}