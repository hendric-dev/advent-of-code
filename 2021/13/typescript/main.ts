import fs from 'fs'

fs.readFile('input.txt', {encoding: 'utf-8'}, (_err, data) => {
  let coordinates: Coordinate[] = buildCoordinates(data.split('\n\n')[0])
  const instructions: Instruction[] = buildInstructions(data.split('\n\n')[1])

  instructions.forEach(({axis, value}, index) => {
    if (axis === 'x') coordinates.forEach(coordinate => coordinate.foldX(Number(value)))
    else coordinates.forEach(coordinate => coordinate.foldY(Number(value)))
    coordinates = removeDuplicates(coordinates)

    if (index === 0) console.log('Dots after first fold', coordinates.length)
  })

  console.log()
  visualize(coordinates, instructions)
})

const buildCoordinates = (input: string): Coordinate[] => (
  input.split('\n').map(line => {
    const [x, y] = line.split(',').map(value => Number(value))
    return new Coordinate(x, y)
  })
)

const buildInstructions = (input: string): Instruction[] => (
  input
    .split('\n')
    .map(line => line.split(' ')[line.split(' ').length - 1])
    .map(instruction => {
      const [axis, value] = instruction.split('=')
      return new Instruction(axis, Number(value))
    })
)

const removeDuplicates = (coordinates: Coordinate[]): Coordinate[] => (
  coordinates.filter((coordinate, index, self) => (
    index === self.findIndex(({x, y}) => x === coordinate.x && y === coordinate.y)
  ))
)

const visualize = (coordinates: Coordinate[], instructions: Instruction[]) => {
  const dimensions = {
    x: instructions.map(x => x).reverse().find(({axis}) => axis === 'x')?.value || 0,
    y: instructions.map(x => x).reverse().find(({axis}) => axis === 'y')?.value || 0,
  };
  [...Array(dimensions.y).keys()].forEach(y => {
    const line = [...Array(dimensions.x).keys()].map(x => {
      if (coordinates.find(coordinate => coordinate.x === x && coordinate.y === y)) return '#'
      else return '.'
    }).join('')
    console.log(line)
  })
}

class Coordinate {
  constructor(public x: number, public y: number) {}

  foldX(value: number) {
    if (this.x >= value) this.x = value - (this.x - value)
  }

  foldY(value: number) {
    if (this.y >= value) this.y = value - (this.y - value)
  }
}

class Instruction {
  constructor(public axis: string, public value: number) {}
}
