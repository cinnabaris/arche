package main

import (
	"os"

	_ "github.com/cinnabaris/arche/plugins/cbeta"
	_ "github.com/cinnabaris/arche/plugins/erp"
	_ "github.com/cinnabaris/arche/plugins/forum"
	_ "github.com/cinnabaris/arche/plugins/hotel"
	_ "github.com/cinnabaris/arche/plugins/mall"
	"github.com/cinnabaris/arche/plugins/nut"
	_ "github.com/cinnabaris/arche/plugins/ops/mail"
	_ "github.com/cinnabaris/arche/plugins/ops/vpn"
	_ "github.com/cinnabaris/arche/plugins/pos"
	_ "github.com/cinnabaris/arche/plugins/survey"
	_ "github.com/cinnabaris/arche/plugins/todo"
	log "github.com/sirupsen/logrus"
)

func main() {
	if err := nut.Main(os.Args...); err != nil {
		log.Fatal(err)
	}
}
