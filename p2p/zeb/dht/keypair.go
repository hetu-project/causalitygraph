package dht

import (
	"golang.org/x/crypto/ed25519"
)

type KeyPair struct {
	PrivateKey ed25519.PrivateKey
	PublicKey  ed25519.PublicKey
}

func (k *KeyPair) Id() []byte {
	return k.PublicKey
}

func GenerateKeyPair(seed []byte) (KeyPair, error) {
	var pubKey ed25519.PublicKey
	var privKey ed25519.PrivateKey
	var err error

	if len(seed) > 0 {
		privKey = ed25519.NewKeyFromSeed(seed)
		pubKey = privKey.Public().(ed25519.PublicKey)
	} else {
		pubKey, privKey, err = ed25519.GenerateKey(nil)
	}

	k := KeyPair{
		PrivateKey: privKey,
		PublicKey:  pubKey,
	}

	return k, err
}
