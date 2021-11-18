package main

import (
	"fmt"
	"github.com/biogo/hts/bgzf"
	"github.com/brentp/vcfgo"
	"os"
)

func main() {

	f, err := os.Open(os.Args[1])
	if err != nil {
		panic(err)
	}

	bgz, err := bgzf.NewReader(f, 1)
	if err != nil {
		panic(err)
	}

	defer bgz.Close()
	defer f.Close()

	rdr, err := vcfgo.NewReader(bgz, true)
	if err != nil {
		panic(err)
	}

	values := make([]int64, 0, 2048)
	for {
		variant := rdr.Read()
		if variant == nil {
			break
		}
		v, err := variant.Info().Get("AN")
		if err != nil {
			continue
		}
		values = append(values, int64(v.(int)))
	}

	s := float64(0)
	for _, v := range values {
		s += float64(v)
	}

	if rdr.Error() != nil {
		panic(rdr.Error())
	}
	fmt.Fprintf(os.Stderr, "%.3f", s/float64(len(values)))
}
