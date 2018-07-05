package i18n

// http://www.gnu.org/software/gettext/manual/gettext.html#Language-Codes
import (
	"bytes"
	"context"
	"errors"
	"fmt"
	"html/template"
	"math"
	"net/http"
	"os"
	"path/filepath"
	"time"

	"github.com/cinnabaris/arche/web/cache"
	"github.com/go-ini/ini"
	"github.com/go-pg/pg"
	log "github.com/sirupsen/logrus"
	"github.com/urfave/negroni"
	"golang.org/x/text/language"
)

const (
	// LOCALE locale key
	LOCALE = "locale"
)

// Locale locale
type Locale struct {
	tableName struct{} `sql:"locales"`

	ID        uint      `json:"id"`
	Lang      string    `json:"lang"`
	Code      string    `json:"code"`
	Message   string    `json:"message"`
	UpdatedAt time.Time `json:"updatedAt"`
	CreatedAt time.Time `json:"createdAt"`
}

// TableName table name
func (p Locale) TableName() string {
	return "locales"
}

// I18n i18n
type I18n struct {
	DB    *pg.DB      `inject:""`
	Cache cache.Cache `inject:""`
}

// Middleware locale middleware
func (p *I18n) Middleware() (negroni.HandlerFunc, error) {
	var tags []language.Tag
	for _, l := range p.Languages {
		t, e := language.Parse(l)
		if e != nil {
			return nil, e
		}
		tags = append(tags, t)
	}
	matcher := language.NewMatcher(tags)
	return func(wrt http.ResponseWriter, req *http.Request, next http.HandlerFunc) {
		lang, written := p.detectLocale(req, LOCALE)
		tag, _, _ := matcher.Match(language.Make(lang))
		if lang != tag.String() {
			written = true
			lang = tag.String()
		}
		if written {
			http.SetCookie(wrt, &http.Cookie{
				Name:     LOCALE,
				Value:    lang,
				MaxAge:   math.MaxInt32,
				Path:     "/",
				Secure:   req.TLS != nil,
				HttpOnly: false,
			})
		}
		ctx := context.WithValue(req.Context(), LOCALE, lang)
		next(wrt, req)
	}, nil
}

func (p *I18n) detectLocale(r *http.Request, k string) (string, bool) {
	// 1. Check URL arguments.
	if lang := r.URL.Query().Get(k); lang != "" {
		return lang, true
	}

	// 2. Get language information from cookies.
	if ck, er := r.Cookie(k); er == nil {
		return ck.Value, false
	}

	// 3. Get language information from 'Accept-Language'.
	return r.Header.Get("Accept-Language"), true
}

// Languages language tags
// func (p *I18n) Languages(db *gorm.DB) ([]string, error) {
// 	var langs []string
// 	if err := db.Model(&Locale{}).Pluck("DISTINCT(lang)", &langs).Error; err != nil {
// 		return nil, err
// 	}
// 	return langs, nil
// }

// Sync sync from filesystem to database
func (p *I18n) Sync(dir string) error {
	const ext = ".ini"
	return filepath.Walk(dir, func(path string, info os.FileInfo, err error) error {
		if err != nil {
			return err
		}
		name := info.Name()
		if info.IsDir() || filepath.Ext(name) != ext {
			return err
		}
		tag, err := language.Parse(name[:len(name)-len(ext)])
		if err != nil {
			return err
		}
		lang := tag.String()
		log.Info("find locale ", lang)

		cfg, err := ini.Load(path)
		if err != nil {
			return err
		}

		tx := p.DB.Begin()
		for k, v := range cfg.Section("").KeysHash() {
			var c int
			tx.Model(&Locale{}).Where("lang = ? AND code = ?", lang, k).Count(&c)
			if c == 0 {
				if err := tx.Create(&Locale{
					Lang:      lang,
					Code:      k,
					Message:   v,
					UpdatedAt: time.Now(),
				}).Error; err != nil {
					tx.Rollback()
					return err
				}
			}
		}
		tx.Commit()

		return nil
	})
}

func (p *I18n) cacheKey(lang, code string) string {
	return "locales/" + lang + "/" + code
}

// Set set
func (p *I18n) Set(db *gorm.DB, lang, code, message string) error {
	var it Locale
	now := time.Now()
	err := db.Select([]string{"id"}).
		Where("lang = ? AND code = ?", lang, code).First(&it).Error
	if err == nil {
		err = db.Model(&it).Update("message", message).Error
	} else if err == gorm.ErrRecordNotFound {
		err = db.Create(&Locale{
			Lang:      lang,
			Code:      code,
			Message:   message,
			UpdatedAt: now,
		}).Error
	}

	if err == nil {
		p.Cache.Set(p.cacheKey(lang, code), message, p.ttl())
	}
	return err
}

// H html
func (p *I18n) H(lang, code string, obj interface{}) (string, error) {
	msg, err := p.get(lang, code)
	if err != nil {
		return "", err
	}
	tpl, err := template.New("").Parse(msg)
	if err != nil {
		return "", err
	}
	var buf bytes.Buffer
	err = tpl.Execute(&buf, obj)
	return buf.String(), err
}

//E error
func (p *I18n) E(lang, code string, args ...interface{}) error {
	msg, err := p.get(lang, code)
	if err != nil {
		return err
	}
	return fmt.Errorf(msg, args...)
}

// All list items by lang
func (p *I18n) All(lang string) (map[string]string, error) {
	var items []Locale
	if err := p.DB.Select([]string{"code", "message"}).
		Where("lang = ?", lang).
		Order("code ASC").Find(&items).Error; err != nil {
		return nil, err
	}
	rst := make(map[string]string)
	for _, it := range items {
		rst[it.Code] = it.Message
	}
	return rst, nil
}

//T text
func (p *I18n) T(lang, code string, args ...interface{}) string {
	msg, err := p.get(lang, code)
	if err != nil {
		return err.Error()
	}
	return fmt.Sprintf(msg, args...)
}

func (p *I18n) ttl() time.Duration {
	return time.Hour * 24 * 7
}

func (p *I18n) get(lang, code string) (string, error) {
	var msg string
	key := p.cacheKey(lang, code)
	if err := p.Cache.Get(key, &msg); err == nil {
		return msg, nil
	}
	var it Locale
	if err := p.DB.Select([]string{"message"}).
		Where("lang = ? AND code = ?", lang, code).
		First(&it).Error; err == nil {
		p.Cache.Set(key, it.Message, p.ttl())
		return it.Message, nil
	}
	return "", errors.New(lang + "." + code)
}
