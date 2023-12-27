const fs = require('fs')

const MATCH = Object.freeze({
  '(': ')',
  '{': '}',
  '[': ']',
  '<': '>',
})

const completeLine = line => {
  const queue = []
  line.split('').forEach(char => {
    if (Object.keys(MATCH).includes(char)) queue.push(char)
    else queue.splice(-1)
  })
  return queue.reverse().map(char => MATCH[char])
}

const illegalCharacters = navigation => navigation.map(line => findIllegalCharacters(line)[0]).filter(value => value)

const incompleteLines = navigation => navigation.filter(line => findIllegalCharacters(line).length === 0)

const findIllegalCharacters = line => {
  const queue = []
  return line.split('').filter(char => {
    if (Object.keys(MATCH).includes(char)) queue.push(char)
    else if (MATCH[queue.splice(-1)] != char) {
      return true
    }
    return false
  })
}

const highScore = chars => (
  chars.reduce((acc, char) => {
    switch (char) {
    case ')': return acc + 3
    case '}': return acc + 1197
    case ']': return acc + 57
    case '>': return acc + 25137
    }
  }, 0)
)

const autocompleteScore = lines => {
  const scores = lines.map(line => {
    // console.log(line)
    return line.reduce((acc, char) => {
      switch (char) {
      case ')': return acc * 5 + 1
      case '}': return acc * 5 + 3
      case ']': return acc * 5 + 2
      case '>': return acc * 5 + 4
      }
    }, 0)
  })
  scores.sort((a, b) => a - b)
  return scores[Math.floor(scores.length / 2)]
}

fs.readFile('input.txt', {encoding: 'utf-8'}, (_, data) => {
  const navigation = data.split('\n')

  console.log(`Found a high score of ${highScore(illegalCharacters(navigation))} for illegal characters`)
  console.log(`Found an autocomplete score of ${autocompleteScore(incompleteLines(navigation).map(completeLine))}`)
})
