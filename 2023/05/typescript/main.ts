import { Category, SeedAlmanac } from './src/seed-almanac'

const input = await Bun.file('input.txt').text()
const almanac = new SeedAlmanac(input)
const locations = almanac.seeds.map((seed) => almanac.convertSeedTo(seed, Category.Location))

console.info(`[Part 1] The lowest possible location is ${Math.min(...locations)}`)
console.info(`[Part 2] The lowest possible location is ${almanac.lowestLocationUsingSeedRanges()}`)
