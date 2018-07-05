package queue

import (
	"fmt"
	"reflect"
	"runtime"
	"time"

	log "github.com/sirupsen/logrus"
)

// Consumer task handler
type Consumer func(id, queue, contentType string, body []byte) error

// Worker worker
type Worker struct {
	consumers map[string]Consumer
}

// Status status
func (p *Worker) Status() map[string]interface{} {
	rst := make(map[string]interface{})
	for k, v := range p.consumers {
		rst[k] = runtime.FuncForPC(reflect.ValueOf(v).Pointer()).Name()
	}
	return rst
}

// Register register handler
func (p *Worker) Register(_type string, hnd Consumer) {
	if _, ok := p.consumers[_type]; ok {
		log.Warn("handler for ", _type, " already exists, will override it")
	}
	p.consumers[_type] = hnd
}

// Do consum a message
func (p *Worker) Do(id, _type, contentType string, body []byte) error {
	log.Info("receive message ", id, "@", _type)
	now := time.Now()
	hnd, ok := p.consumers[_type]
	if !ok {
		return fmt.Errorf("unknown message type %s", _type)
	}
	if err := hnd(id, _type, contentType, body); err != nil {
		log.Error(err)
		return err
	}
	log.Infof("done %s %s", id, time.Now().Sub(now))
	return nil
}
