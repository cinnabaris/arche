package storage

// Storage storage
type Storage interface {
	Write(name string, body []byte, size int64) (fileType string, url string, err error)
}
