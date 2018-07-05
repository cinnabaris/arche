package aes

import (
	"crypto/aes"
	"crypto/cipher"
	"crypto/rand"

	"github.com/cinnabaris/arche/web/crypto"
)

// NewSecurity new security
func New(key []byte) (crypto.SecretBox, error) {
	cip, err := aes.NewCipher(key)
	if err != nil {
		return nil, err
	}
	return &SecretBox{cip: cip}, nil
}

// SecretBox by nacl
type SecretBox struct {
	cip cipher.Block
}

// Encrypt encrypt
func (p *SecretBox) Encrypt(buf []byte) ([]byte, error) {
	iv := make([]byte, aes.BlockSize)
	if _, err := rand.Read(iv); err != nil {
		return nil, err
	}
	cfb := cipher.NewCFBEncrypter(p.cip, iv)
	val := make([]byte, len(buf))
	cfb.XORKeyStream(val, buf)

	return append(val, iv...), nil
}

// Decrypt decrypt
func (p *SecretBox) Decrypt(buf []byte) ([]byte, error) {
	bln := len(buf)
	cln := bln - aes.BlockSize
	ct := buf[0:cln]
	iv := buf[cln:bln]

	cfb := cipher.NewCFBDecrypter(p.cip, iv)
	val := make([]byte, cln)
	cfb.XORKeyStream(val, ct)
	return val, nil
}
