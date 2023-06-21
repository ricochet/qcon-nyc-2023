package main

//go:generate wit-bindgen tiny-go ./gopher.wit -w critter --out-dir=gen

import (
	. "github.com/ricochet/qcon-nyc-2023/gopher/gen"
)

type GopherImpl struct {
}

func init() {
	a := GopherImpl{}
	SetExportsComponentExampleGreeting(a)
}

func (i GopherImpl) Hello(a string) string {
	return a + "\n<🐹> Hey!"
}

func main() {}
