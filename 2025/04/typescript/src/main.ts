import { chain, countBy, range, sum } from "lodash-es"
import { readFile } from "node:fs/promises"

const getAdjacent = (grid: string[][], row: number, col: number): string[] => [
  grid[row - 1]?.[col - 1],
  grid[row - 1]?.[col],
  grid[row - 1]?.[col + 1],
  grid[row]?.[col - 1],
  grid[row]?.[col + 1],
  grid[row + 1]?.[col - 1],
  grid[row + 1]?.[col],
  grid[row + 1]?.[col + 1],
]

const markRemovablePaperRolls = (grid: string[][]): void => {
  range(0, grid.length).forEach((row) =>
    range(0, grid[row].length).forEach((col) => {
      if (grid[row][col] !== "@") return

      if (
        chain(getAdjacent(grid, row, col))
          .filter((v) => ["@", "🎯"].includes(v))
          .size()
          .value() < 4
      ) {
        grid[row][col] = "🎯"
      }
    })
  )
}

const removeMarkedPaperRolls = (grid: string[][]): number => {
  const removable = grid.flat().filter((v) => v === "🎯").length
  grid.forEach((_, index) => (grid[index] = grid[index].map((slot) => (slot === "🎯" ? "x" : slot))))
  return removable
}

const input = await readFile("input.txt", { encoding: "utf8" })
const grid = input.split("\n").map((row) => row.split(""))

const removedPaperRoles: number[] = []
while (removedPaperRoles.at(-1) !== 0) {
  markRemovablePaperRolls(grid)
  const removed = removeMarkedPaperRolls(grid)
  removedPaperRoles.push(removed)
}

console.log(`🏗️ The forklifts can reach ${removedPaperRoles[0]} paper rolls in the first round`)
console.log(`🏗️ The forklifts can reach ${sum(removedPaperRoles)} paper rolls if we keep on removing them`)
