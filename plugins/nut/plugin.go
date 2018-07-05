package nut

import (
	"encoding/base64"

	"github.com/cinnabaris/arche/web"
	"github.com/cinnabaris/arche/web/cache"
	"github.com/facebookgo/inject"
	"github.com/spf13/viper"
	"golang.org/x/text/language"
)

// Plugin plugin
type Plugin struct {
	cache cache.Cache `inject:""`
}

// Init init beans
func (p *Plugin) Init(*inject.Graph) error {
	return nil
}

// Mount register
func (p *Plugin) Mount() error {
	return nil
}

func init() {
	web.Register(&Plugin{})

	viper.SetDefault("languages", []string{
		language.AmericanEnglish.String(),
		language.SimplifiedChinese.String(),
		language.TraditionalChinese.String(),
	})

	viper.SetDefault("aws", map[string]interface{}{
		"access_key_id":     "change-me",
		"secret_access_key": "change-me",
		"s3": map[string]interface{}{
			"region": "change-me",
			"bucket": "change-me",
		},
	})

	viper.SetDefault("redis", map[string]interface{}{
		"host": "localhost",
		"port": 6379,
		"db":   8,
	})

	viper.SetDefault("rabbitmq", map[string]interface{}{
		"user":     "guest",
		"password": "guest",
		"host":     "localhost",
		"port":     5672,
		"virtual":  "arche-dev",
		"queue":    "tasks",
	})

	viper.SetDefault("postgresql", map[string]interface{}{
		"host":     "localhost",
		"port":     5432,
		"user":     "postgres",
		"password": "",
		"name":     "arche_dev",
		"sslmode":  "disable",
	})

	viper.SetDefault("http", map[string]interface{}{
		"port":  8080,
		"name":  "www.change-me.com",
		"theme": "bootstrap",
	})

	secret, _ := web.RandomBytes(32)
	viper.SetDefault("secret", base64.StdEncoding.EncodeToString(secret))

	viper.SetDefault("elasticsearch", []string{"http://localhost:9200"})

}
