package web

import (
	"github.com/facebookgo/inject"
	"github.com/urfave/cli"
)

// Plugin plugin
type Plugin interface {
	// init beans
	Init(*inject.Graph) error
	// console commands
	Shell() []cli.Command
	// register
	Mount() error
}

var plugins []Plugin

// Register register plugins
func Register(args ...Plugin) {
	plugins = append(plugins, args...)
}
