package redis

import (
	"bytes"
	"encoding/gob"
	"time"

	"github.com/cinnabaris/arche/web/cache"
	_redis "github.com/gomodule/redigo/redis"
)

// New new cache
func New(prefix string, pool *_redis.Pool) cache.Cache {
	return &Cache{prefix: prefix, pool: pool}
}

// Cache cache
type Cache struct {
	pool   *_redis.Pool `inject:""`
	prefix string
}

// Get get
func (p *Cache) Get(key string, val interface{}) error {
	c := p.pool.Get()
	defer c.Close()
	bys, err := _redis.Bytes(c.Do("GET", p.prefix+key))
	if err != nil {
		return err
	}
	var buf bytes.Buffer
	dec := gob.NewDecoder(&buf)
	buf.Write(bys)
	return dec.Decode(val)
}

// Set set
func (p *Cache) Set(key string, val interface{}, ttl time.Duration) error {
	var buf bytes.Buffer
	enc := gob.NewEncoder(&buf)
	if err := enc.Encode(val); err != nil {
		return err
	}

	c := p.pool.Get()
	defer c.Close()
	_, err := c.Do("SET", p.prefix+key, buf.Bytes(), "EX", int(ttl/time.Second))
	return err
}

// Flush clear cache
func (p *Cache) Flush() error {
	c := p.pool.Get()
	defer c.Close()
	keys, err := _redis.Values(c.Do("KEYS", p.prefix+"*"))
	if err == nil && len(keys) > 0 {
		_, err = c.Do("DEL", keys...)
	}
	return err
}

// Keys cache keys
func (p *Cache) Keys() ([]string, error) {
	c := p.pool.Get()
	defer c.Close()
	keys, err := _redis.Strings(c.Do("KEYS", p.prefix+"*"))
	if err != nil {
		return nil, err
	}
	for i := range keys {
		keys[i] = keys[i][len(p.prefix):]
	}
	return keys, nil
}
