//go:build !libsecp256k1

package nostr

import (
	"encoding/hex"
	"fmt"
    "strings"

	"github.com/ethereum/go-ethereum/crypto"
	//"github.com/ethereum/go-ethereum/common/hexutil"
)

// CheckSignature checks if the event signature is valid for the given event.
// It won't look at the ID field, instead it will recompute the id from the entire event body.
// If the signature is invalid bool will be false and err will be set.
func (evt Event) CheckSignature() (bool, error) {
	// read and check pubkey
    address := "0x" + evt.PubKey

	// read signature
    //sig, err := hexutil.Decode(evt.Sig)
    sig, err := hex.DecodeString(evt.Sig)
	if err != nil {
		return false, fmt.Errorf("signature '%s' is invalid hex: %w", evt.Sig, err)
	}
	if sig[64] >= 27 {
		sig[64] -= 27
	}

	// check signature
    message := evt.Serialize()
    prefixedMessage := fmt.Sprintf("\x19Ethereum Signed Message:\n%d%s", len(message), message)
	hash := crypto.Keccak256Hash([]byte(prefixedMessage))

    pubKey, err := crypto.SigToPub(hash.Bytes(), sig)
	if err != nil {
        return false, fmt.Errorf("failed to recover public key: %w", err)
	}

	recoveredAddr := crypto.PubkeyToAddress(*pubKey).Hex()

	return (recoveredAddr == address), nil
}

// Sign signs an event with a given privateKey.
// It sets the event's ID, PubKey, and Sig fields.
// Returns an error if the private key is invalid or if signing fails.
func (evt *Event) Sign(secretKey string) error {
    s, err := crypto.HexToECDSA(secretKey)
	if err != nil {
		return fmt.Errorf("Sign called with invalid secret key '%s': %w", secretKey, err)
	}

	if evt.Tags == nil {
		evt.Tags = make(Tags, 0)
	}

	evt.PubKey = crypto.PubkeyToAddress(s.PublicKey).Hex()
    evt.PubKey = strings.TrimPrefix(crypto.PubkeyToAddress(s.PublicKey).Hex(), "0x")


    message := evt.Serialize()
	prefixedMessage := fmt.Sprintf("\x19Ethereum Signed Message:\n%d%s", len(message), message)
	h:= crypto.Keccak256Hash([]byte(prefixedMessage))

    sig, err := crypto.Sign(h.Bytes(), s)
	if err != nil {
        return fmt.Errorf("failed to sign: %w", err)
	}

	evt.ID = hex.EncodeToString(h.Bytes())
    //evt.Sig = hexutil.Encode(sig)
	evt.Sig = hex.EncodeToString(sig)

	return nil
}
