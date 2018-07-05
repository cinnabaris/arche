package nut

import (
	"crypto/rand"
	"crypto/rsa"
	"crypto/x509"
	"crypto/x509/pkix"
	"encoding/pem"
	"math/big"
	"os"
	"path"
	"time"
)

//WritePemFile wirte to a pem format file
func WritePemFile(file, _type string, buf []byte, mode os.FileMode) error {
	if err := os.MkdirAll(path.Dir(file), 0755); err != nil {
		return err
	}
	fd, err := os.OpenFile(file, os.O_WRONLY|os.O_CREATE|os.O_EXCL, mode)
	if err != nil {
		return err
	}
	defer fd.Close()
	return pem.Encode(
		fd,
		&pem.Block{
			Type:  _type,
			Bytes: buf,
		},
	)
}

//CreateCertificate create certs
func CreateCertificate(ca bool, subject pkix.Name, years int) ([]byte, []byte, error) {
	now := time.Now()

	serialNumber, err := rand.Int(
		rand.Reader,
		new(big.Int).Lsh(big.NewInt(1), 128))
	if err != nil {
		return nil, nil, err
	}
	//http://golang.org/pkg/crypto/x509/#Certificate
	tpl := &x509.Certificate{
		SerialNumber: serialNumber,
		IsCA:         ca,
		BasicConstraintsValid: true,
		SubjectKeyId:          []byte(now.Format("20060102150405")),
		Subject:               subject,
		NotBefore:             now,
		NotAfter:              now.AddDate(years, 0, 0),
		SignatureAlgorithm:    x509.SHA512WithRSA,
		//http://golang.org/pkg/crypto/x509/#KeyUsage
		ExtKeyUsage: []x509.ExtKeyUsage{x509.ExtKeyUsageClientAuth, x509.ExtKeyUsageServerAuth},
		KeyUsage:    x509.KeyUsageDigitalSignature | x509.KeyUsageCertSign,
	}

	key, err := rsa.GenerateKey(rand.Reader, 2048)
	if err != nil {
		return nil, nil, err
	}

	cert, err := x509.CreateCertificate(
		rand.Reader,
		tpl,
		tpl,
		&key.PublicKey,
		key,
	)
	if err != nil {
		return nil, nil, err
	}

	return x509.MarshalPKCS1PrivateKey(key), cert, nil
}
