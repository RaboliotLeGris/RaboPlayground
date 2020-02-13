package main

import (
	"math/rand"
	"strings"
)

type World struct {
	grid []int
	w    int
	h    int
}

func NewWorld(w, h int) World {
	grid := make([]int, w*h)
	for i := range grid {
		grid[i] = genCell()
	}

	return World{
		grid: grid,
		w:    w,
		h:    h,
	}
}

func NewWorldFromGrid(grid []int, w, h int) World {
	return World{
		grid: grid,
		w:    w,
		h:    h,
	}
}

func (w *World) Print() string {
	var str strings.Builder
	for i, k := range w.grid {
		if i%w.w == 0 {
			str.WriteString("\n")
		}
		if k == 1 {
			str.WriteString("X")
		} else {
			str.WriteString(".")
		}
	}
	str.WriteString("\n")
	return str.String()
}

func (w *World) Simulate() {
	nGrid := make([]int, w.w * w.h);
	for i, k := range w.grid {
		x := i % w.w
		y := i / w.w
		nbNeighbour := w.countNeighbour(x, y)
		if k == 1 && (nbNeighbour == 2 || nbNeighbour == 3) {
			nGrid[i] = 1
		} else if k == 1 && (nbNeighbour < 2 || nbNeighbour > 3) {
			nGrid[i] = 0
		} else if k == 0 && nbNeighbour == 3 {
			nGrid[i] = 1
		}
	}
	w.grid = nGrid
}

func (w *World) countNeighbour(x, y int) int {
	count := 0
	for _, col := range []int{x - 1, x, x + 1} {
		for _, row := range []int{y - 1, y, y + 1} {
			if col == x && row == y {
				continue
			}
			if col == -1 {
				col = w.w - 1
			}
			if row == -1 {
				row = w.h - 1
			}
			count += w.grid[w.w*(row%w.w)+(col%w.h)]
		}
	}
	return count
}

func genCell() int {
	if rand.Float32() > 0.7 {
		return 1
	}
	return 0
}
