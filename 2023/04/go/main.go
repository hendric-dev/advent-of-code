package main

import "fmt"

func main() {
	lines, _ := readFile("input.txt")
	scratchcards := parse_scratchcards(lines)
	part1(scratchcards)
	part2(scratchcards)
}

func part1(scratchcards []ScratchCard) {
	totalPoints := 0
	for _, scratchcard := range scratchcards {
		points := 0
		for _, number := range scratchcard.numbers {
			if contains(scratchcard.winningNumbers, number) {
				if points == 0 {
					points = 1
				} else {
					points *= 2
				}
			}
		}
		totalPoints += points
	}
	fmt.Println("[Part 1] Total points:", totalPoints)
}

func part2(scratchcards []ScratchCard) {
	for index, scratchcard := range scratchcards {
		matches := 0
		for _, number := range scratchcard.numbers {
			if contains(scratchcard.winningNumbers, number) {
				matches++
			}
		}
		for i := index + 1; i <= index+matches; i++ {
			if len(scratchcards) > i {
				scratchcards[i].copies += 1 + scratchcard.copies
			}
		}
	}

	count := 0
	for _, scratchcard := range scratchcards {
		count += scratchcard.copies + 1
	}
	fmt.Println("[Part 2] Total amount of scratchcards:", count)
}
