package amqp

import (
	"bytes"
	"encoding/gob"
	"time"

	"github.com/cinnabaris/arche/web/queue"
	"github.com/google/uuid"
	log "github.com/sirupsen/logrus"
	_amqp "github.com/streadway/amqp"
)

// New new queue
func New(url, queue string) (*Queue, error) {
	it := &Queue{url: url, queue: queue}
	if err := it.open(func(ch *_amqp.Channel) error {
		_, err := ch.QueueDeclare(queue, true, false, false, false, nil)
		if err != nil {
			return err
		}
		return nil
	}); err != nil {
		return nil, err
	}
	return it, nil
}

// Queue amqp queue
type Queue struct {
	url   string
	queue string
}

// Receive receive
func (p *Queue) Receive(consumer string, worker *queue.Worker) error {
	log.Info("waiting for messages, to exit press CTRL+C")
	return p.open(func(ch *_amqp.Channel) error {
		if err := ch.Qos(1, 0, false); err != nil {
			return err
		}
		msgs, err := ch.Consume(p.queue, consumer, false, false, false, false, nil)
		if err != nil {
			return err
		}
		for d := range msgs {
			d.Ack(false)
			if err := worker.Do(d.MessageId, d.Type, d.ContentType, d.Body); err != nil {
				return err
			}
		}
		return nil
	})
}

// Send send job
func (p *Queue) Send(_type string, priority uint8, body interface{}) error {
	var buf bytes.Buffer
	enc := gob.NewEncoder(&buf)
	err := enc.Encode(body)
	if err != nil {
		return err
	}
	return p.open(func(ch *_amqp.Channel) error {
		return ch.Publish("", p.queue, false, false, _amqp.Publishing{
			DeliveryMode: _amqp.Persistent,
			MessageId:    uuid.New().String(),
			Priority:     priority,
			Body:         buf.Bytes(),
			Timestamp:    time.Now(),
			Type:         _type,
		})
	})
}

func (p *Queue) open(f func(*_amqp.Channel) error) error {
	conn, err := _amqp.Dial(p.url)
	if err != nil {
		return err
	}
	defer conn.Close()
	ch, err := conn.Channel()
	if err != nil {
		return err
	}
	defer ch.Close()
	return f(ch)
}
