const fs = require('fs')

class Octopus {
  flashed = false

  constructor(row, column, energy) {
    this.row = row
    this.column = column
    this.energy = energy
  }

  increaseEnergy(amount) {
    this.energy += amount
  }

  readyToFlash() {
    return this.energy > 9 && !this.flashed
  }

  reset() {
    this.flashed = false
    if (this.energy > 9) this.energy = 0
  }
}

class Octopuses {
  constructor(input) {
    this.map = {}
    input.split("\n").forEach((line, row) => {
      line.split("").forEach((energy, column) => {
        this.map[`${row}.${column}`] = new Octopus(row, column, Number(energy))
      })
    })
  }

  get flashes() {
    return Object.values(this.map).filter(({ flashed }) => flashed).length
  }

  get isSimultaneousFlash() {
    return this.flashes === Object.values(this.map).length
  }

  advance() {
    Object.values(this.map).forEach(octopus => octopus.increaseEnergy(1))
    Object.values(this.map).forEach(octopus => this.flash(octopus))
  }

  flash(octopus) {
    if (!octopus.readyToFlash()) return

    octopus.flashed = true
    const surroundingOctopuses = this.getSurroundingOctopuses(octopus)
    surroundingOctopuses.forEach(surroundingOctopus => surroundingOctopus.increaseEnergy(1))
    surroundingOctopuses.forEach(surroundingOctopus => this.flash(surroundingOctopus))
  }

  getSurroundingOctopuses({ row, column }) {
    const topLeft = this.map[`${row - 1}.${column - 1}`]
    const top = this.map[`${row - 1}.${column}`]
    const topRight = this.map[`${row - 1}.${column + 1}`]
    const left = this.map[`${row}.${column - 1}`]
    const right = this.map[`${row}.${column + 1}`]
    const bottomLeft = this.map[`${row + 1}.${column - 1}`]
    const bottom = this.map[`${row + 1}.${column}`]
    const bottomRight = this.map[`${row + 1}.${column + 1}`]

    return [topLeft, top, topRight, left, right, bottomLeft, bottom, bottomRight].filter(octopus => octopus)
  }

  reset() {
    Object.values(this.map).forEach(octopus => octopus.reset())
  }
}

fs.readFile('input.txt', { encoding: 'utf-8' }, (_, input) => {
  const octopuses = new Octopuses(input)
  let flashes = 0
  let simultaneousFlash = undefined
  let step = 1

  while (!simultaneousFlash || step <= 100) {
    octopuses.advance()
    if (step <= 100) flashes += octopuses.flashes
    if (!simultaneousFlash && octopuses.isSimultaneousFlash) simultaneousFlash = step
    octopuses.reset()
    step += 1
  }

  console.log(`There were ${flashes} octopuses flashing after 100 steps.`)
  console.log(`Calculated a simultaneous flash after ${simultaneousFlash} steps.`)
})
