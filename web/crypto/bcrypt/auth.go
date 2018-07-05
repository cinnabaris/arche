package bcrypt

import (
	"github.com/cinnabaris/arche/web/crypto"
	"golang.org/x/crypto/bcrypt"
)

// New create auth
func New() crypto.Auth {
	return &Auth{}
}

// Auth by bcrypt
type Auth struct{}

// Sum sum a hash
func (p *Auth) Sum(plain []byte) ([]byte, error) {
	return bcrypt.GenerateFromPassword(plain, 16)
}

// Verify check hash
func (p *Auth) Verify(encode, plain []byte) bool {
	return bcrypt.CompareHashAndPassword(encode, plain) == nil
}
