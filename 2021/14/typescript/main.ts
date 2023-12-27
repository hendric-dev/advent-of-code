import fs from 'fs'

type Pairs = Record<string, number>
type Quantity = Record<string, number>
type Rules = Record<string, string>

fs.readFile('input.txt', {encoding: 'utf-8'}, (_err, input) => {
  const polymer = new Polymer(input.split('\n')[0].split(''));
  const rules: Rules = input.split('\n')
    .filter(value => value.includes('->'))
    .reduce<Rules>((result, rule) => {
      const [key, value] = rule.split(' -> ')
      result[key] = value
      return result
    }, {});

  [...Array(10).keys()].forEach(() => polymer.step(rules))
  console.log('Polymer score after 10 steps:', polymer.score());
  [...Array(30).keys()].forEach(() => polymer.step(rules))
  console.log('Polymer score after 40 steps:', polymer.score())
})

class Polymer {
  pairs: Pairs
  quantities: Quantity

  constructor(template: string[]) {
    this.pairs = template.reduce<Pairs>((result, value, index) => {
      if (index === template.length - 1) return result

      const pair = value + template[index + 1]
      result[pair] ||= 0
      result[pair] += 1
      return result
    }, {})
    this.quantities = template.reduce<Quantity>((result, char) => {
      result[char] ||= 0
      result[char] += 1
      return result
    }, {})
  }

  score(): number {
    const values = Object.values(this.quantities)
    values.sort((a, b) => a - b)
    return values[values.length - 1] - values[0]
  }

  step(rules: Rules) {
    this.pairs = Object.keys(this.pairs)
      .filter(pair => this.pairs[pair] > 0)
      .reduce<Pairs>((result, pair) => {
        const occurances = this.pairs[pair]
        const first = pair[0] + rules[pair]
        const second = rules[pair] + pair[1]
        result[first] ||= 0
        result[first] += occurances
        result[second] ||= 0
        result[second] += occurances
        this.quantities[rules[pair]] ||= 0
        this.quantities[rules[pair]] += occurances

        return result
      }, {})
  }
}
