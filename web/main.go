package web

import (
	"fmt"
	"runtime"
	"sort"
	"time"

	"github.com/urfave/cli"
)

// Main entry
func Main(args ...string) error {

	app := cli.NewApp()
	app.Name = args[0]
	app.Version = fmt.Sprintf("%s (%s) by %s", Version, BuildTime, runtime.Version())
	app.Authors = []cli.Author{
		cli.Author{
			Name:  AuthorName,
			Email: AuthorEmail,
		},
	}
	if ts, err := time.Parse(time.RFC1123Z, BuildTime); err == nil {
		app.Compiled = ts
	}

	app.Copyright = Copyright
	app.Usage = Usage
	app.EnableBashCompletion = true
	app.Commands = make([]cli.Command, 0)
	for _, p := range plugins {
		app.Commands = append(app.Commands, p.Shell()...)
	}

	sort.Sort(cli.FlagsByName(app.Flags))
	sort.Sort(cli.CommandsByName(app.Commands))

	return app.Run(args)

}
