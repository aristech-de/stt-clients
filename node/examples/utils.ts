import { type ConnectionOptions } from '@aristech-org/stt-client'
import fs from 'fs'

const { HOST: host, SSL: useSsl, ROOT_CERT: rootCert, TOKEN: token, SECRET: secret } = process.env
const auth = token && secret ? { token, secret } : undefined
// If a root certificate is provided or authentication is used, SSL is enforced
// otherwise we use the value of the SSL environment variable
const ssl = !!rootCert || useSsl === 'true'
if (!host) {
  console.error('Please provide the HOST environment variable')
  process.exit(1)
}

export function getClientOptions(): ConnectionOptions {
  return { host, ssl, rootCert, auth }
}

export const getWaveFilePath = () => {
  const waveFilePath = process.argv[2]
  if (!waveFilePath || !fs.existsSync(waveFilePath) || !waveFilePath.endsWith('.wav')) {
    console.error('Please provide the path to a wave file as the first argument')
    process.exit(1)
  }
  return waveFilePath
}

export function getWaveFile() {
  const waveFilePath = getWaveFilePath()
  // Get a readable stream from the wave file
  const waveFile = fs.createReadStream(waveFilePath)
  // Skip the wave header
  waveFile.read(44)
  return waveFile
}

/**
 * Reads the command line argument behind `--name` or returns the default value
 */
export function getArg(name: string, defaultValue: string) {
  const index = process.argv.indexOf(`--${name}`)
  if (index >= 0 && index < process.argv.length) {
    return process.argv[index + 1]
  }
  return defaultValue
}