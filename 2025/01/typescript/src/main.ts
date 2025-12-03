import { chain } from "lodash-es"
import { readFile } from "node:fs/promises"

const input = await readFile("input.txt", { encoding: "utf8" })
const rotations = input.replaceAll("R", "+").replaceAll("L", "-").split("\n").map(Number)

const sequence = [{ rotation: 0, start: 50, stop: 50, zeroPassed: 0 }]
rotations.map((rotation) => {
  const start = sequence.at(-1)?.stop ?? 0
  const rotated = start + (rotation % 100)
  const stop = (rotated + 100) % 100

  sequence.push({
    rotation,
    start,
    stop,
    zeroPassed: Math.floor(Math.abs(rotation) / 100) + (start !== 0 && (rotated <= 0 || rotated >= 100) ? 1 : 0),
  })
})

const password1 = chain(sequence)
  .filter(({ stop }) => stop === 0)
  .size()
  .value()
const password2 = chain(sequence).map("zeroPassed").sum().value()

console.log(`🔓 Password unlocked -> ${password1}`)
console.log(`🔓 Password unlocked for real this time -> ${password2}`)
