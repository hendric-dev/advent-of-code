import { max, range, sum } from "lodash-es"
import { readFile } from "node:fs/promises"

const getHighestJoltage = (batteries: number[], activate: number): number => {
  let lastBatteryIndex = -1
  return Number(
    range(activate - 1, -1)
      .map((remainingToBeActivated) => {
        const remainingBatteries =
          remainingToBeActivated === 0
            ? batteries.slice(lastBatteryIndex + 1)
            : batteries.slice(lastBatteryIndex + 1, -remainingToBeActivated)
        const battery = max(remainingBatteries)
        if (!battery) return

        lastBatteryIndex += remainingBatteries.indexOf(battery) + 1
        return battery
      })
      .join("")
  )
}

const input = await readFile("input.txt", { encoding: "utf8" })
const banks = input.split("\n")
const joltage1 = banks.map((bank) => {
  const batteries = bank.split("").map(Number)
  return getHighestJoltage(batteries, 2)
})
const joltage2 = banks.map((bank) => {
  const batteries = bank.split("").map(Number)
  return getHighestJoltage(batteries, 12)
})

console.log(`🪫 Switching on 2 batteries...  the max possible joltage is ${sum(joltage1)}`)
console.log(`🔋 Switching on 12 batteries... the max possible joltage is ${sum(joltage2)}`)
