package cache

import "time"

// Cache cache
type Cache interface {
	Get(key string, val interface{}) error
	Set(key string, val interface{}, ttl time.Duration) error
	Flush() error
	Keys() ([]string, error)
}
