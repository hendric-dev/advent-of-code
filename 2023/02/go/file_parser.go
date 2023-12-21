package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

type GameData struct {
	GameID   int
	MaxBlue  int
	MaxGreen int
	MaxRed   int
}

func parseLine(line string) (GameData, error) {
	parts := strings.Split(line, ":")
	if len(parts) < 2 {
		return GameData{}, fmt.Errorf("invalid line format")
	}

	gameID, err := strconv.Atoi(strings.TrimSpace(parts[0][5:]))
	if err != nil {
		return GameData{}, err
	}

	colorParts := strings.Split(parts[1], ";")
	maxBlue, maxGreen, maxRed := 0, 0, 0
	for _, part := range colorParts {
		colors := strings.Split(part, ",")
		for _, color := range colors {
			color = strings.TrimSpace(color)
			if strings.Contains(color, "blue") {
				value, _ := strconv.Atoi(strings.Split(color, " ")[0])
				if value > maxBlue {
					maxBlue = value
				}
			} else if strings.Contains(color, "green") {
				value, _ := strconv.Atoi(strings.Split(color, " ")[0])
				if value > maxGreen {
					maxGreen = value
				}
			} else if strings.Contains(color, "red") {
				value, _ := strconv.Atoi(strings.Split(color, " ")[0])
				if value > maxRed {
					maxRed = value
				}
			}
		}
	}

	return GameData{
		GameID:   gameID,
		MaxBlue:  maxBlue,
		MaxGreen: maxGreen,
		MaxRed:   maxRed,
	}, nil
}

func readFile(filename string) ([]GameData, error) {
	file, err := os.Open(filename)
	if err != nil {
		return nil, err
	}
	defer file.Close()

	var games []GameData
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		game, err := parseLine(scanner.Text())
		if err != nil {
			return nil, err
		}
		games = append(games, game)
	}

	if err := scanner.Err(); err != nil {
		return nil, err
	}

	return games, nil
}
