package crypto

// Auth auth authenticates a message
type Auth interface {
	Sum(plain []byte) ([]byte, error)
	Verify(encode, plain []byte) bool
}
