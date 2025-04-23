import { finalizeEvent, generateSecretKey, getPublicKey } from '../lib/esm/pure.js'
import { Relay, useWebSocketImplementation } from '../lib/esm/relay.js'
import WebSocket from 'ws'

useWebSocketImplementation(WebSocket)

const relay = await Relay.connect('ws://161.97.129.166:10547')
console.log(`connected to ${relay.url}`)

// let's publish a new event while simultaneously monitoring the relay for it
let sk = generateSecretKey()
let pk = getPublicKey(sk)

relay.subscribe([
  {
    kinds: [1],
    authors: [pk],
  },
], {
  onevent(event) {
    console.log('got event:', event)
  }
})

let eventTemplate = {
  kind: 1,
  created_at: Math.floor(Date.now() / 1000),
  tags: [],
  content: 'hello blake',
}

// this assigns the pubkey, calculates the event id and signs the event in a single step
const signedEvent = finalizeEvent(eventTemplate, sk)
await relay.publish(signedEvent)

relay.close()