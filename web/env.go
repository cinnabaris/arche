package web

import (
	"encoding/base64"
	"log/syslog"
	"reflect"

	"github.com/facebookgo/inject"
	log "github.com/sirupsen/logrus"
	logrus_syslog "github.com/sirupsen/logrus/hooks/syslog"
	"github.com/spf13/viper"
	"github.com/urfave/cli"
)

// H hash
type H map[string]interface{}

const (
	// PRODUCTION production mode
	PRODUCTION = "production"
)

// MODE run mode
func MODE() string {
	return viper.GetString("env")
}

// SECRET secret key 32-bits
func SECRET() ([]byte, error) {
	return base64.StdEncoding.DecodeString(viper.GetString("secret"))
}

type injectLogger struct {
}

func (p *injectLogger) Debugf(format string, v ...interface{}) {
	log.Debugf(format, v...)
}

// ConfigAction read config at first
func ConfigAction(f cli.ActionFunc) cli.ActionFunc {
	viper.SetEnvPrefix(reflect.TypeOf(Main).String())
	viper.BindEnv("env")

	viper.SetConfigName("config")
	viper.SetConfigType("toml")
	viper.AddConfigPath(".")

	return func(c *cli.Context) error {
		if err := viper.ReadInConfig(); err != nil {
			return err
		}
		if MODE() == PRODUCTION {
			// ----------
			log.SetLevel(log.InfoLevel)
			wrt, err := syslog.New(syslog.LOG_INFO, viper.GetString("server.name"))
			if err != nil {
				return err
			}
			log.AddHook(&logrus_syslog.SyslogHook{Writer: wrt})
		} else {
			log.SetLevel(log.DebugLevel)
		}

		log.Infof("read config from %s", viper.ConfigFileUsed())
		return f(c)
	}
}

// InjectAction open beans at first
func InjectAction(f cli.ActionFunc) cli.ActionFunc {
	return ConfigAction(func(c *cli.Context) error {
		gh := inject.Graph{Logger: &injectLogger{}}
		for _, p := range plugins {
			if err := p.Init(&gh); err != nil {
				return err
			}
			if err := gh.Provide(&inject.Object{Value: p}); err != nil {
				return err
			}
		}
		if err := gh.Populate(); err != nil {
			return err
		}
		for _, p := range plugins {
			if err := p.Mount(); err != nil {
				return err
			}
		}
		return f(c)
	})
}

func init() {
	viper.SetDefault("env", "development")
}
