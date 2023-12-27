const fs = require('fs')

const partOne = (directions => {
  const position = directions.reduce((result, [direction, distance]) => {
    if (direction === 'forward') result.forward += distance
    else if (direction === 'up') (result.depth - distance) < 0 ? result.depth = 0 : result.depth -= distance
    else result.depth += distance

    return result
  }, {forward: 0, depth: 0})

  console.log(`[Part One] Horizontal position: ${position.forward}`)
  console.log(`[Part One] Depth: ${position.depth}`)
  console.log(`[Part One] Multiplied: ${position.forward * position.depth}`)
})

const partTwo = (directions => {
  const position = directions.reduce((result, [direction, distance]) => {
    if (direction === 'forward') {
      result.forward += distance
      result.depth += distance * result.aim
    }
    else if (direction === 'up') result.aim -= distance
    else result.aim += distance

    return result
  }, {forward: 0, depth: 0, aim: 0})

  console.log(`[Part Two] Horizontal position: ${position.forward}`)
  console.log(`[Part Two] Depth: ${position.depth}`)
  console.log(`[Part Two] Multiplied: ${position.forward * position.depth}`)
})

fs.readFile('input.txt', {encoding: 'utf-8'}, (_, data) => {
  const directions = data.split('\n')
    .map(value => value.split(' '))
    .map(([direction, distance]) => [direction, Number(distance)])

  partOne(directions)
  partTwo(directions)
})
