package queue

// Queue messaging queue
type Queue interface {
	Send(_type string, priority uint8, body interface{})
	Receive(consumer string, worker *Worker) error
}
