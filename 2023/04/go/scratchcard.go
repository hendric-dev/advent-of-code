package main

import (
	"fmt"
	"strconv"
	"strings"
)

type ScratchCard struct {
	numbers        []int
	winningNumbers []int
	copies         int
}

func parse_scratchcards(lines []string) []ScratchCard {
	scratchcards := []ScratchCard{}
	for _, line := range lines {
		scratchcard, err := parse_scratchcard(line)
		if err != nil {
			fmt.Println("Error parsing scratchcard")
			panic(err)
		}
		scratchcards = append(scratchcards, scratchcard)
	}
	return scratchcards
}

func parse_scratchcard(line string) (ScratchCard, error) {
	split := strings.Split(strings.Split(line, ":")[1], "|")
	rawWinningNumbers := strings.TrimSpace(split[0])
	rawNumbers := strings.TrimSpace(split[1])
	numbers := []int{}
	winningNumbers := []int{}

	for _, rawWinningNumber := range strings.Split(rawWinningNumbers, " ") {
		number, err := strconv.Atoi(rawWinningNumber)
		if rawWinningNumber == "" {
			continue
		}
		if err != nil {
			return ScratchCard{}, err
		}
		winningNumbers = append(winningNumbers, number)
	}

	for _, rawNumber := range strings.Split(rawNumbers, " ") {
		number, err := strconv.Atoi(rawNumber)
		if err != nil {
			if rawNumber == "" {
				continue
			}
			fmt.Println("Error parsing number")
			return ScratchCard{}, err
		}
		numbers = append(numbers, number)
	}

	return ScratchCard{numbers: numbers, winningNumbers: winningNumbers, copies: 0}, nil
}
