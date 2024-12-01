import { chain, countBy } from "lodash-es";

const input = await Bun.file("./input.txt").text();

const [first, second] = chain(input)
  .split(/\n|\s+/)
  .map<[number, number]>((location, index) => [Number(location), index])
  .partition(([_, index]) => index % 2 === 0)
  .value()
  .map((list) => list.map(([location, _]) => location))
  .map((list) => list.sort());

const distance = chain(first)
  .zip(second)
  .map(([a = 0, b = 0]) => Math.abs(a - b))
  .sum()
  .value();

console.info(
  `[Part 1] 🤖 Beep, Boop... calculated a total distance of ${distance}`
);

const counts = countBy(second, (location) => location);
const similarityScore = chain(first)
  .map((location) => location * (counts[location] ?? 0))
  .sum()
  .value();

console.info(
  `[Part 2] 🤖 Beep, Boop... calculated a similarity score of ${similarityScore}`
);
