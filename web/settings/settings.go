package web

import (
	"bytes"
	"encoding/gob"
	"time"

	"github.com/jinzhu/gorm"
)

// Setting setting model
type Setting struct {
	ID        uint `gorm:"primary_key"`
	Key       string
	Value     []byte
	Encode    bool
	UpdatedAt time.Time
	CreatedAt time.Time
}

// TableName table name
func (p Setting) TableName() string {
	return "settings"
}

// Settings settings
type Settings struct {
	Security *Security `inject:""`
}

// Set set
func (p *Settings) Set(db *gorm.DB, key string, obj interface{}, encode bool) error {
	var buf bytes.Buffer
	enc := gob.NewEncoder(&buf)
	err := enc.Encode(obj)
	if err != nil {
		return err
	}
	var val []byte
	if encode {
		if val, err = p.Security.Encrypt(buf.Bytes()); err != nil {
			return err
		}
	} else {
		val = buf.Bytes()
	}

	var it Setting
	now := time.Now()
	err = db.Select([]string{"id"}).Where("key = ?", key).First(&it).Error
	if err == nil {
		err = db.Model(&it).Updates(map[string]interface{}{
			"value":  val,
			"encode": encode,
		}).Error
	} else if err == gorm.ErrRecordNotFound {
		err = db.Create(&Setting{
			Key:       key,
			Value:     val,
			Encode:    encode,
			UpdatedAt: now,
		}).Error
	}

	return err
}

// Get get
func (p *Settings) Get(db *gorm.DB, key string, obj interface{}) error {
	var it Setting
	if err := db.Select([]string{"value", "encode"}).
		Where("key = ?", key).
		First(&it).Error; err != nil {
		return err
	}

	var buf bytes.Buffer
	dec := gob.NewDecoder(&buf)

	if it.Encode {
		vl, err := p.Security.Decrypt(it.Value)
		if err != nil {
			return err
		}
		buf.Write(vl)
	} else {
		buf.Write(it.Value)
	}

	return dec.Decode(obj)
}
