package main

import (
	"fmt"
	"strconv"
)

type EngineSchematic struct {
	value   int
	symbols []Symbol
}

type Symbol struct {
	value    string
	position string
}

func parse_engine_schematics(lines []string) ([]EngineSchematic, error) {
	symbols := getAllSymbols(lines)
	var engineSchematics []EngineSchematic
	for row, line := range lines {
		temp := ""
		start := 0
		for column, char := range line {
			endOfLine := column == len(line)-1
			if char >= '0' && char <= '9' {
				if temp == "" {
					start = column
				}
				temp += string(char)
				if !endOfLine {
					continue
				}
			}

			if temp != "" {
				value, _ := strconv.Atoi(temp)
				engineSchematic := EngineSchematic{
					value:   value,
					symbols: getAdjacentSymbols(symbols, row, start, column-1),
				}
				engineSchematics = append(engineSchematics, engineSchematic)
				temp = ""
			}
		}
	}

	return engineSchematics, nil
}

func getAllSymbols(lines []string) map[string]string {
	symbols := make(map[string]string)
	for row, line := range lines {
		for column, character := range line {
			if isSymbol(character) {
				symbols[fmt.Sprintf("%d.%d", row, column)] = string(character)
			}
		}
	}
	return symbols
}

func getAdjacentSymbols(symbols map[string]string, row, start, end int) []Symbol {
	result := []Symbol{}
	for irow := row - 1; irow <= row+1; irow++ {
		for icol := start - 1; icol <= end+1; icol++ {
			if irow == row && icol >= start && icol <= end {
				continue
			}
			position := fmt.Sprintf("%d.%d", irow, icol)
			symbol, exists := symbols[position]
			if exists {
				result = append(result, Symbol{value: symbol, position: position})
			}
		}
	}

	return result
}

func isSymbol(char rune) bool {
	_, err := strconv.Atoi(string(char))
	return err != nil && char != '.'
}
