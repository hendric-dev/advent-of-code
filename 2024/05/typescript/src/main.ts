import { chain, partition, sumBy } from "lodash-es"

const input = await Bun.file("./input.txt").text()
let [orderInput, updatesInput] = input.split("\n\n")

const order = orderInput.split("\n").reduce<Record<number, number[]>>((result, rule) => {
  const [a, b] = rule.split("|").map(Number)
  result[a] ??= []
  result[a].push(b)
  return result
}, {})

const updates = updatesInput.split("\n").map((update) => update.split(",").map(Number))
const [correctUpdates, incorrectUpdates] = partition(updates, (update) => {
  const previous: number[] = []
  return update.every((page) => {
    const isBreakingRule = previous.some((previousPage) => (order[page] ?? []).includes(previousPage))
    previous.push(page)
    return !isBreakingRule
  })
})

const middleElement = (array: number[]): number => array.at(array.length / 2) ?? 0
let sum = sumBy(correctUpdates, middleElement)

console.info(`[Part 1] 🤖 Beep, Boop... the sum of correct updates is ${sum}`)

const fixOrder = (array: number[]): number[] => array.sort((a, b) => ((order[a] ?? []).includes(b) ? -1 : 1))
sum = chain(incorrectUpdates).map(fixOrder).sumBy(middleElement).value()

console.info(`[Part 2] 🤖 Beep, Boop... the sum of fixed incorrect updates is ${sum}`)
