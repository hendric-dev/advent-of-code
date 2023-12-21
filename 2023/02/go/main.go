package main

import (
	"fmt"
	"log"
)

func main() {
	games, err := readFile("input.txt")
	if err != nil {
		log.Fatal(err)
	}
	part1(games)
	part2(games)
}

func part1(games []GameData) {
	validationGameData := GameData{
		MaxBlue:  14,
		MaxGreen: 13,
		MaxRed:   12,
	}

	sumOfGameIDs := 0
	for _, game := range games {
		if game.MaxBlue <= validationGameData.MaxBlue && game.MaxGreen <= validationGameData.MaxGreen && game.MaxRed <= validationGameData.MaxRed {
			sumOfGameIDs += game.GameID
		}
	}

	fmt.Printf("[Part 1] Sum of Game IDs for valid games: %d\n", sumOfGameIDs)
}

func part2(games []GameData) {
	sumOfPower := 0
	for _, game := range games {
		product := game.MaxRed * game.MaxGreen * game.MaxBlue
		sumOfPower += product
	}

	fmt.Printf("[Part 2] Sum of power for each game: %d\n", sumOfPower)
}
