const fs = require('fs')

fs.readFile('input.txt', {encoding: 'utf-8'}, (_, input) => {
  const submarines = input.split(',').map(submarine => Number(submarine))
  const min = Math.min(...submarines)
  const max = Math.max(...submarines)
  const range = [...Array(max - min).keys()].map(value => value + min)

  console.log(
    'Lowest with steady fuel costs:',
    Math.min(...range.map(position => submarines.reduce((acc, submarine) => acc + Math.abs(submarine - position), 0))),
  )

  console.log(
    'Lowest with increasing fuel costs:',
    Math.min(...range.map(position => submarines.reduce((acc, submarine) => {
      const distance = Math.abs(submarine - position)
      return acc + (Math.pow(distance, 2) + distance) / 2
    }, 0))),
  )
})
