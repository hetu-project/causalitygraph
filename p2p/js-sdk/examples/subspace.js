import { finalizeEvent, generateSecretKey, getPublicKey } from '../lib/esm/pure.js';
import { Relay, useWebSocketImplementation } from '../lib/esm/relay.js';
import WebSocket from 'ws';
import {
    NewSubspaceCreateEvent,
    ValidateSubspaceCreateEvent,
    NewSubspaceJoinEvent,
    ValidateSubspaceJoinEvent,
    NewSubspaceOpEvent,
    SetContentType,
    toNostrEvent,
    KindSubspaceCreate,
    ValidateSubspaceOpEvent,
} from '../lib/esm/subspace.js';

useWebSocketImplementation(WebSocket);

// Connect to the relay for test
const relayURL = 'ws://161.97.129.166:10547'

const relay = await Relay.connect(relayURL);
console.log(`connected to ${relay.url}`);

// Generate keys
let sk = generateSecretKey();
let pk = getPublicKey(sk);

// Subscribe to events
// This will listen for subspace creation events from the specified public key
relay.subscribe([
    {
      kinds: [KindSubspaceCreate],
      limit: 1,
    },
  ], {
    onevent(event) {
        console.log('====================================');
        console.log('Subscribe got event:', event);
    }
})

// Create a new subspace
const subspaceName = 'TestSubspace';
const ops = 'post=1,propose=2,vote=3,invite=4';
const rules = 'rule1';
const description = 'This is a test subspace';
const imageURL = 'http://example.com/image.png';

const subspaceEvent = NewSubspaceCreateEvent(subspaceName, ops, rules, description, imageURL);
ValidateSubspaceCreateEvent(subspaceEvent);

// Sign and publish the subspace creation event
const signedSubspaceEvent = finalizeEvent(toNostrEvent(subspaceEvent), sk);
await relay.publish(signedSubspaceEvent);
console.log('====================================');
console.log('Subspace creation event published:', signedSubspaceEvent);

// Join the subspace
const joinEvent = NewSubspaceJoinEvent(subspaceEvent.subspaceID);
ValidateSubspaceJoinEvent(joinEvent);

// Sign and publish the subspace join event
const signedJoinEvent = finalizeEvent(toNostrEvent(joinEvent), sk);
await relay.publish(signedJoinEvent);
console.log('====================================');
console.log('Subspace join event published:', signedJoinEvent);

// Perform an operation in the subspace
const opEvent = NewSubspaceOpEvent(subspaceEvent.subspaceID, 'post');
SetContentType(opEvent, 'text/plain');
opEvent.content = 'This is a post in the subspace.';
ValidateSubspaceOpEvent(opEvent);

// Sign and publish the subspace operation event
const signedOpEvent = finalizeEvent(toNostrEvent(opEvent), sk);
await relay.publish(signedOpEvent);
console.log('====================================');
console.log('Subspace operation event published:', signedOpEvent);

relay.close();