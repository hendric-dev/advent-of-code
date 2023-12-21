package main

import "fmt"

func main() {
	lines, _ := readFile("input.txt")
	engineSchematics, _ := parse_engine_schematics(lines)
	fmt.Println("[Part 1] Sum of part numbers:", part1(engineSchematics)) // 521601
	fmt.Println("[Part 2] The gear ratio is:", part2(engineSchematics))   // 80694070
}

func part1(engineSchematics []EngineSchematic) int {
	sum := 0
	for _, engineSchematic := range engineSchematics {
		isPartNumber := len(engineSchematic.symbols) > 0
		if isPartNumber {
			sum += engineSchematic.value
		}
	}
	return sum
}

func part2(engineSchematics []EngineSchematic) int {
	gear := make(map[string][]int)
	ratio := 0
	for _, engineSchematic := range engineSchematics {
		gearPositions := []string{}
		for _, symbol := range engineSchematic.symbols {
			if symbol.value == "*" {
				gearPositions = append(gearPositions, symbol.position)
			}
		}
		for _, position := range gearPositions {
			gear[position] = append(gear[position], engineSchematic.value)
		}
	}
	for _, values := range gear {
		if len(values) == 2 {
			ratio += values[0] * values[1]
		}
	}
	return ratio
}
