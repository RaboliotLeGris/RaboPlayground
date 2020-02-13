package main

import (
	"fmt"
	"math/rand"
	"testing"
)

const oSit = "\nX.......X.\n.XX...X.XX\n..........\n...X......\n..X.X.XX.X\n.X.X.X.X..\n....XX...X\n..X...X.XX\n.X.XXX.X.X\n.....X....\n"
const pSit = "\nXXX....XX.\n..X....X.X\n..........\n...X......\n..X.XX....\n...X..X...\n...XXXX.XX\n..X......X\nXXXXXXX..X\n.....X.X..\n"

func TestWorld_Print(t *testing.T) {
	rand.Seed(0)
	w := NewWorld(10, 10)
	if w.Print() != oSit {
		fmt.Printf("Got \t: %q\nExpected: %q\n", w.Print(), oSit)
		t.Fail()
	}
}

func TestWorld_Simulate(t *testing.T) {
	rand.Seed(0)
	w := NewWorld(10, 10)
	w.Simulate()
	if w.Print() != pSit {
		fmt.Printf("Got \t: %q\nExpected: %q\n", w.Print(), pSit)
		t.Fail()
	}
}
