package crypto

// SecretBox encrypt and descrypt message
type SecretBox interface {
	Encrypt(buf []byte) ([]byte, error)
	Decrypt(buf []byte) ([]byte, error)
}
