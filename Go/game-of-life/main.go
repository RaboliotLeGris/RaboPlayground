package main

import (
	"fmt"
	"math/rand"
)

func main() {
	rand.Seed(0) //time.Now().UnixNano())

	w := NewWorld(10, 10)
	itNb := 20
	for i := 0; i < itNb; i++{
		fmt.Println("Iteration nb:", i)
		fmt.Print(w.Print())
		w.Simulate()
	}
}
