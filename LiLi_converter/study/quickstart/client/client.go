package main

import (
	"asynq-quickstart/task"
	"log"
	"time"

	"github.com/hibiken/asynq"
)

type SVC struct {
	asynq *asynq.Client
}

func init() {
	svc.asynq = asynq.NewClient(asynq.RedisClientOpt{Addr: "localhost:6379"})
}

var svc SVC

// client.go
func main() {
	t1, err := task.NewReminderEmailTask(42)
	if err != nil {
		log.Fatal(err)
	}

	t2, err := task.NewWelcomeEmailTask(42)
	if err != nil {
		log.Fatal(err)
	}
	// Process the task immediately.

	if info, err := svc.asynq.Enqueue(t1); err != nil {
		log.Fatal(err)
	} else {
		log.Printf(" [*] Successfully enqueued task: %+v", info)
	}

	// Process the task 24 hours later.
	if info, err := svc.asynq.Enqueue(t2, asynq.ProcessIn(24*time.Hour), asynq.Retention(24*time.Hour)); err != nil {
		log.Fatal(err)
	} else {
		log.Printf(" [*] Successfully enqueued task: %+v", info)
	}

}
