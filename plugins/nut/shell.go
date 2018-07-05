package nut

import (
	"crypto/x509/pkix"
	"fmt"
	"html/template"
	"io/ioutil"
	"os"
	"path"
	"path/filepath"
	"time"

	"github.com/BurntSushi/toml"
	"github.com/cinnabaris/arche/web"
	"github.com/go-pg/pg"
	log "github.com/sirupsen/logrus"
	"github.com/spf13/viper"
	"github.com/urfave/cli"
	"golang.org/x/text/language"
)

// Shell console commands
func (p *Plugin) Shell() []cli.Command {
	return []cli.Command{
		{
			Name:    "generate",
			Aliases: []string{"g"},
			Usage:   "generate file template",
			Subcommands: []cli.Command{
				{
					Name:    "config",
					Aliases: []string{"c"},
					Usage:   "generate config file",
					Flags: []cli.Flag{
						cli.StringFlag{
							Name:  "environment, e",
							Value: "development",
							Usage: "environment, like: development, production, stage, test...",
						},
					},
					Action: p.generateConfig,
				},
				{
					Name:    "nginx",
					Aliases: []string{"ng"},
					Usage:   "generate nginx.conf",
					Flags: []cli.Flag{
						cli.BoolFlag{
							Name:  "https, s",
							Usage: "use https?",
						},
					},
					Action: web.ConfigAction(p.generateNginxConf),
				},
				{
					Name:    "openssl",
					Aliases: []string{"ssl"},
					Usage:   "generate ssl certificates",
					Flags: []cli.Flag{
						cli.StringFlag{
							Name:  "name, n",
							Usage: "name",
						},
						cli.StringFlag{
							Name:  "country, c",
							Value: "Earth",
							Usage: "country",
						},
						cli.StringFlag{
							Name:  "organization, o",
							Value: "Mother Nature",
							Usage: "organization",
						},
						cli.IntFlag{
							Name:  "years, y",
							Value: 1,
							Usage: "years",
						},
					},
					Action: p.generateSsl,
				},
				{
					Name:    "migration",
					Usage:   "generate migration file",
					Aliases: []string{"m"},
					Flags: []cli.Flag{
						cli.StringFlag{
							Name:  "name, n",
							Usage: "name",
						},
					},
					Action: p.generateMigration,
				},
				{
					Name:    "locale",
					Usage:   "generate locale file",
					Aliases: []string{"l"},
					Flags: []cli.Flag{
						cli.StringFlag{
							Name:  "name, n",
							Usage: "locale name",
						},
					},
					Action: p.generateLocale,
				},
			},
		},
		{
			Name:    "database",
			Aliases: []string{"db"},
			Usage:   "database operations",
			Subcommands: []cli.Command{
				{
					Name:    "example",
					Usage:   "scripts example for create database and user",
					Aliases: []string{"e"},
					Action:  web.ConfigAction(p.databaseExample),
				},
				{
					Name:    "migrate",
					Usage:   "migrate the DB to the most recent version available",
					Aliases: []string{"m"},
					Action: p.databaseRun(func(items ...Migration) error {
						db := p.openDatabase()
						tx, err := db.Begin()
						if err != nil {
							return err
						}
						defer tx.Rollback()
						if _, err = tx.Exec(
							"CREATE TABLE IF NOT EXISTS schema_migrations (version VARCHAR(255) PRIMARY KEY,	created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT NOW())",
						); err != nil {
							return err
						}
						for _, it := range items {
							var count int
							_, err := tx.QueryOne(pg.Scan(&count), "SELECT count(*) FROM schema_migrations WHERE version = ?", it.Name)
							if err != nil {
								return err
							}
							if count == 0 {
								log.Infof("run migration %s", it.Name)
								if _, err := tx.Exec(it.Up); err != nil {
									return err
								}
								if _, err := tx.Exec(`insert into schema_migrations(version) values(?)`, it.Name); err != nil {
									return err
								}
							}
						}

						return tx.Commit()
					}),
				},
				{
					Name:    "rollback",
					Usage:   "roll back the version by 1",
					Aliases: []string{"r"},
					Action: p.databaseRun(func(items ...Migration) error {
						db := p.openDatabase()
						tx, err := db.Begin()
						if err != nil {
							return err
						}
						defer tx.Rollback()
						var version string
						if _, err := tx.QueryOne(pg.Scan(&version), "SELECT version FROM schema_migrations ORDER BY version DESC LIMIT 1"); err != nil {
							return err
						}
						for _, it := range items {
							if it.Name == version {
								log.Infof("rollback migration %s", it.Name)
								if _, err := tx.Exec(it.Down); err != nil {
									return err
								}
								_, err := tx.QueryOne(pg.Scan(&version), "DELETE FROM schema_migrations WHERE version = ?", version)
								if err != nil {
									return err
								}
							}
						}

						return tx.Commit()
					}),
				},
				{
					Name:    "version",
					Usage:   "dump the migration status for the current DB",
					Aliases: []string{"v"},
					Action: p.databaseRun(func(items ...Migration) error {
						db := p.openDatabase()
						tpl := "%-36s %s\n"
						fmt.Printf(tpl, "NAME", "RUN AT")
						for _, it := range items {
							var at time.Time
							if _, err := db.QueryOne(pg.Scan(&at), "SELECT created_at FROM schema_migrations WHERE version = ? LIMIT 1", it.Name); err == nil {
								fmt.Printf(tpl, it.Name, at.Format(time.RFC1123))
							}
						}
						return nil
					}),
				},
				{
					Name:    "connect",
					Usage:   "connect database",
					Aliases: []string{"c"},
					Action:  web.ConfigAction(p.connectDatabase),
				},
				{
					Name:    "create",
					Usage:   "create database",
					Aliases: []string{"n"},
					Action:  web.ConfigAction(p.createDatabase),
				},
				{
					Name:    "drop",
					Usage:   "drop database",
					Aliases: []string{"d"},
					Action:  web.ConfigAction(p.dropDatabase),
				},
			},
		},
		{
			Name:    "server",
			Aliases: []string{"s"},
			Usage:   "start the app server",
			// Action: web.InjectAction(func(_ *cli.Context) error {
			// 	go func() {
			// 		// ----------
			// 		host, err := os.Hostname()
			// 		if err != nil {
			// 			log.Error(err)
			// 		}
			// 		for {
			// 			if err := p.Jobber.Receive(host); err != nil {
			// 				log.Error(err)
			// 				time.Sleep(5 * time.Second)
			// 			}
			// 		}
			// 	}()
			// 	// -------
			// 	return p.startServer(viper.GetInt("server.port"), viper.GetString("env") == web.PRODUCTION)
			// }),
		},
		{
			Name:    "routes",
			Aliases: []string{"rt"},
			Usage:   "print out all defined routes",
			Action: web.InjectAction(func(_ *cli.Context) error {
				tpl := "%-16s %s\n"
				fmt.Printf(tpl, "METHODS", "PATH")
				// for _, rt := range p.Router.Routes() {
				// 	fmt.Printf(tpl, rt.Method, rt.Path)
				// }
				return nil
			}),
		},
		{
			Name:  "i18n",
			Usage: "internationalization operations",
			Subcommands: []cli.Command{
				{
					Name:    "sync",
					Usage:   "sync locales from locales to database",
					Aliases: []string{"s"},
					// Action: web.InjectAction(func(_ *cli.Context) error {
					// 	return p.I18n.Sync("locales")
					// }),
				},
			},
		},
	}
}

// --------------------------------------------

func (p *Plugin) generateNginxConf(c *cli.Context) error {
	pwd, err := os.Getwd()
	if err != nil {
		return err
	}
	name := viper.GetString("http.name")

	tpl, err := template.ParseFiles(path.Join("templates", "nginx.conf"))
	if err != nil {
		return err
	}

	fn := path.Join("tmp", name+".conf")
	if err = os.MkdirAll(path.Dir(fn), 0700); err != nil {
		return err
	}
	fmt.Printf("generate file %s\n", fn)
	fd, err := os.OpenFile(fn, os.O_WRONLY|os.O_CREATE|os.O_EXCL, 0600)
	if err != nil {
		return err
	}
	defer fd.Close()

	return tpl.Execute(fd, struct {
		Port  int
		Root  string
		Name  string
		Theme string
		Ssl   bool
	}{
		Name:  name,
		Port:  viper.GetInt("http.port"),
		Root:  pwd,
		Theme: viper.GetString("http.theme"),
		Ssl:   c.Bool("https"),
	})
}

func (p *Plugin) generateSsl(c *cli.Context) error {
	name := c.String("name")
	if len(name) == 0 {
		cli.ShowCommandHelp(c, "openssl")
		return nil
	}
	root := path.Join("tmp", "etc", "ssl", name)

	key, crt, err := CreateCertificate(
		true,
		pkix.Name{
			Country:      []string{c.String("country")},
			Organization: []string{c.String("organization")},
		},
		c.Int("years"),
	)
	if err != nil {
		return err
	}

	fnk := path.Join(root, "key.pem")
	fnc := path.Join(root, "crt.pem")

	fmt.Printf("generate pem file %s\n", fnk)
	err = WritePemFile(fnk, "RSA PRIVATE KEY", key, 0600)
	fmt.Printf("test: openssl rsa -noout -text -in %s\n", fnk)

	if err == nil {
		fmt.Printf("generate pem file %s\n", fnc)
		err = WritePemFile(fnc, "CERTIFICATE", crt, 0444)
		fmt.Printf("test: openssl x509 -noout -text -in %s\n", fnc)
	}
	if err == nil {
		fmt.Printf(
			"verify: diff <(openssl rsa -noout -modulus -in %s) <(openssl x509 -noout -modulus -in %s)",
			fnk,
			fnc,
		)
	}
	fmt.Println()
	return nil
}

func (p *Plugin) generateLocale(c *cli.Context) error {
	name := c.String("name")
	if len(name) == 0 {
		cli.ShowCommandHelp(c, "locale")
		return nil
	}
	lng, err := language.Parse(name)
	if err != nil {
		return err
	}
	const root = "locales"
	if err = os.MkdirAll(root, 0700); err != nil {
		return err
	}
	file := path.Join(root, fmt.Sprintf("%s.ini", lng.String()))
	fmt.Printf("generate file %s\n", file)
	fd, err := os.OpenFile(file, os.O_WRONLY|os.O_CREATE|os.O_EXCL, 0600)
	if err != nil {
		return err
	}
	defer fd.Close()
	return err
}

func (p *Plugin) migrationsDir() string {
	return filepath.Join("db", "migrations")
}
func (p *Plugin) generateMigration(c *cli.Context) error {
	name := c.String("name")
	if len(name) == 0 {
		cli.ShowCommandHelp(c, "migration")
		return nil
	}

	version := time.Now().Format("20060102150405")
	root := filepath.Join(p.migrationsDir(), fmt.Sprintf("%s_%s", version, name))
	if err := os.MkdirAll(root, 0700); err != nil {
		return err
	}
	for _, v := range []string{"up", "down"} {
		fn := filepath.Join(root, fmt.Sprintf("%s.sql", v))
		fmt.Printf("generate file %s\n", fn)
		fd, err := os.OpenFile(fn, os.O_WRONLY|os.O_CREATE|os.O_EXCL, 0600)
		if err != nil {
			return err
		}
		defer fd.Close()
	}
	return nil
}

func (p *Plugin) generateConfig(c *cli.Context) error {
	const fn = "config.toml"
	if _, err := os.Stat(fn); err == nil {
		return fmt.Errorf("file %s already exists", fn)
	}
	fmt.Printf("generate file %s\n", fn)

	viper.Set("env", c.String("environment"))

	fd, err := os.OpenFile(fn, os.O_WRONLY|os.O_CREATE|os.O_TRUNC, 0600)
	if err != nil {
		return err
	}
	defer fd.Close()

	enc := toml.NewEncoder(fd)
	return enc.Encode(viper.AllSettings())
}

func (p *Plugin) databaseExample(_ *cli.Context) error {
	args := viper.GetStringMapString("postgresql")
	fmt.Printf("CREATE USER %s WITH PASSWORD '%s';\n", args["user"], args["password"])
	fmt.Printf("CREATE DATABASE %s WITH ENCODING='UTF8';\n", args["name"])
	fmt.Printf("GRANT ALL PRIVILEGES ON DATABASE %s TO %s;\n", args["name"], args["user"])
	return nil
}

// Migration migration model
type Migration struct {
	Name string
	Up   string
	Down string
}

func (p *Plugin) openDatabase() *pg.DB {
	args := viper.GetStringMapString("postgresql")
	db := pg.Connect(&pg.Options{
		User:     args["user"],
		Password: args["password"],
		Database: args["name"],
		Addr:     args["host"] + ":" + args["port"],
	})
	// db.OnQueryProcessed(func(event *pg.QueryProcessedEvent) {
	// 	query, _ := event.FormattedQuery()
	// 	log.Debugf("%s %s", time.Since(event.StartTime), query)
	// })
	return db
}
func (p *Plugin) databaseRun(f func(...Migration) error) cli.ActionFunc {
	return web.ConfigAction(func(_ *cli.Context) error {
		root := p.migrationsDir()
		files, err := ioutil.ReadDir(root)
		if err != nil {
			return err
		}
		var items []Migration
		for _, f := range files {
			it := Migration{Name: f.Name()}
			log.Debug("find migration ", it.Name)
			if buf, err := ioutil.ReadFile(filepath.Join(root, it.Name, "up.sql")); err == nil {
				it.Up = string(buf)
			} else {
				return err
			}
			if buf, err := ioutil.ReadFile(filepath.Join(root, it.Name, "down.sql")); err == nil {
				it.Down = string(buf)
			} else {
				return err
			}

			items = append(items, it)
		}
		return f(items...)
	})
}

func (p *Plugin) createDatabase(_ *cli.Context) error {
	args := viper.GetStringMapString("postgresql")
	return web.Shell("psql",
		"-h", args["host"],
		"-p", args["port"],
		"-U", "postgres",
		"-c", fmt.Sprintf(
			"CREATE DATABASE %s WITH ENCODING='UTF8'",
			args["name"],
		),
	)
}

func (p *Plugin) dropDatabase(_ *cli.Context) error {
	args := viper.GetStringMapString("postgresql")
	return web.Shell("psql",
		"-h", args["host"],
		"-p", args["port"],
		"-U", "postgres",
		"-c", fmt.Sprintf("DROP DATABASE %s", args["name"]),
	)
}

func (p *Plugin) connectDatabase(_ *cli.Context) error {
	args := viper.GetStringMapString("postgresql")
	return web.Shell("psql",
		"-h", args["host"],
		"-p", args["port"],
		"-U", args["user"],
		args["name"],
	)
}

//
// func (p *Plugin) startServer(port int, grace bool) error {
//
// 	log.Infof(
// 		"application starting on http://localhost:%d",
// 		port,
// 	)
//
// 	srv := &http.Server{
// 		Addr: fmt.Sprintf(":%d", port),
// 		Handler: cors.New(cors.Options{
// 			AllowedOrigins: viper.GetStringSlice("server.frontend"),
// 			AllowedMethods: []string{
// 				http.MethodGet,
// 				http.MethodPost,
// 				http.MethodPatch,
// 				http.MethodPut,
// 				http.MethodDelete,
// 			},
// 			AllowedHeaders:   []string{"Authorization", "X-Requested-With"},
// 			AllowCredentials: true,
// 			Debug:            web.MODE() != web.PRODUCTION,
// 		}).Handler(p.Router),
// 		// Handler: csrf.Protect(
// 		// 	secret,
// 		// 	csrf.Path("/"),
// 		// 	csrf.Secure(secure),
// 		// 	csrf.CookieName("csrf"),
// 		// 	csrf.RequestHeader("Authenticity-Token"),
// 		// 	csrf.FieldName("authenticity_token"),
// 		// )(p.Router),
// 	}
//
// 	if !grace {
// 		return srv.ListenAndServe()
// 	}
//
// 	go func() {
// 		// service connections
// 		if err := srv.ListenAndServe(); err != nil {
// 			log.Error(err)
// 		}
// 	}()
//
// 	// Wait for interrupt signal to gracefully shutdown the server with
// 	// a timeout of 5 seconds.
// 	quit := make(chan os.Signal)
// 	signal.Notify(quit, os.Interrupt)
// 	<-quit
// 	log.Warn("shutdown server ...")
//
// 	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
// 	defer cancel()
// 	if err := srv.Shutdown(ctx); err != nil {
// 		return err
// 	}
// 	log.Warn("server exiting")
// 	return nil
// }
