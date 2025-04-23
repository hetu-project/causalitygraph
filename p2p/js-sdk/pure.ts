import { schnorr, secp256k1 } from '@noble/curves/secp256k1'
import { bytesToHex } from '@noble/hashes/utils'
import { Nostr, Event, EventTemplate, UnsignedEvent, VerifiedEvent, verifiedSymbol, validateEvent } from './core.ts'
import { keccak_256 } from '@noble/hashes/sha3'
import { addr } from 'micro-eth-signer'
import * as typed from 'micro-eth-signer/typed-data'
import { utf8Encoder } from './utils.ts'

class JS implements Nostr {
  generateSecretKey(): Uint8Array {
    return secp256k1.utils.randomPrivateKey()
  }
  getPublicKey(secretKey: Uint8Array): string {
    return bytesToHex(secp256k1.getPublicKey(secretKey))
  }
  finalizeEvent(t: EventTemplate, secretKey: Uint8Array): VerifiedEvent {
    const event = t as VerifiedEvent

    const privKey = bytesToHex(secretKey)
    const pubKey = secp256k1.getPublicKey(secretKey)
    event.pubkey = addr.fromPublicKey(pubKey).slice(2)

    event.id = getEventHash(event)

    const message = serializeEvent(event)
    event.sig = typed.personal.sign(message, privKey).slice(2)
    //event[verifiedSymbol] = true

    return event
  }
  verifyEvent(event: Event): event is VerifiedEvent {
    if (typeof event[verifiedSymbol] === 'boolean') return event[verifiedSymbol]

    const hash = getEventHash(event)
    if (hash !== event.id) {
      //event[verifiedSymbol] = false
      return false
    }
    const address = '0x' + event.pubkey
    const message = serializeEvent(event)
    //const signature = '0x' + event.sig

    const sigBytes = Buffer.from(event.sig, 'hex')

    if ((sigBytes[64] == 1) || (sigBytes[64] == 0)) {
        sigBytes[64] += 27
    }
    const signature = '0x' + sigBytes.toString('hex')

    try {
      const valid = typed.personal.verify(signature, message, address)
      //const valid = schnorr.verify(event.sig, hash, event.pubkey)
      //event[verifiedSymbol] = valid
      return valid
    } catch (err) {
      //event[verifiedSymbol] = false
      return false
    }
  }
}

export function serializeEvent(evt: UnsignedEvent): string {
  if (!validateEvent(evt)) throw new Error("Can't serialize event with wrong or missing properties")
  return JSON.stringify([0, evt.pubkey, evt.created_at, evt.kind, evt.tags, evt.content])
}

export function getEventHash(event: UnsignedEvent): string {
  const message = serializeEvent(event)
  const prefixedMessage = `\x19Ethereum Signed Message:\n${message.length}${message}`
  const eventHash = keccak_256(utf8Encoder.encode(prefixedMessage))

  return bytesToHex(eventHash)
}

const i: JS = new JS()

export const generateSecretKey = i.generateSecretKey
export const getPublicKey = i.getPublicKey
export const finalizeEvent = i.finalizeEvent
export const verifyEvent = i.verifyEvent
export * from './core.ts'
