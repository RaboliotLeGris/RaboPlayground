package main

func Empty(w, h int) []int {
	return make([]int, w*h)
}

func Glider(w, h int) []int {
	g := make([]int, w*h)

	g[1] = 1
	g[w+2] = 1
	g[2*w] = 1
	g[2*w+1] = 1
	g[2*w+2] = 1

	return g
}

func Beacon(w, h int) []int {
	g := make([]int, w*h)

	g[0] = 1
	g[1] = 1
	g[w] = 1
	g[2*w+3] = 1
	g[3*w+2] = 1
	g[3*w+3] = 1

	return g
}