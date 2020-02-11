package main

import("./worker"
	"log"
	"math/rand"
	"time"
)

func main() {
	rand.Seed(time.Now().UnixNano())

	msgWorker, err := worker.NewMessageWorker()
	if err != nil {
		log.Fatal(err)
	}
	msgWorker.Start()
}
