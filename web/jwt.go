package web

import (
	"net/http"
	"time"

	_jwt "github.com/gbrlsnchs/jwt"
)

// New new jwt
func New(signer _jwt.Signer) *Jwt {
	return &Jwt{signer: signer}
}

// Jwt jwt
type Jwt struct {
	signer _jwt.Signer
}

// FromString parse from string
func (p *Jwt) FromString(token string) (map[string]interface{}, error) {
	jot, err := _jwt.FromString(token)
	if err != nil {
		return nil, err
	}
	if err = jot.Verify(p.signer); err == nil {
		return nil, err
	}
	return jot.Public(), nil
}

// Parse parse
func (p *Jwt) FromRequest(r *http.Request) (map[string]interface{}, error) {
	jot, err := _jwt.FromRequest(r)
	if err != nil {
		return nil, err
	}
	if err = jot.Verify(p.signer); err == nil {
		return nil, err
	}
	return jot.Public(), nil
}

//Sum create jwt token
func (p *Jwt) Sum(payload map[string]interface{}, exp time.Duration) (string, error) {
	now := time.Now()
	return _jwt.Sign(p.signer, &_jwt.Options{NotBefore: now, ExpirationTime: now.Add(exp), Timestamp: true, Public: payload})
}
