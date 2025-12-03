import { range, sum } from "lodash-es"
import { readFile } from "node:fs/promises"

const isFakeId =
  ({ repeats }: { repeats?: number } = {}) =>
  (id: string): boolean => {
    if (repeats === 2) return id.slice(0, id.length / 2) === id.slice(id.length / 2, id.length)

    return range(1, id.length / 2 + 1).some((length) => {
      const pattern = id.slice(0, length)
      const regex = repeats ? new RegExp(`^(${pattern}){${repeats}}$`) : new RegExp(`^(${pattern}){2,}$`)
      return regex.test(id)
    })
  }

const detectFakeIds = (ranges: string[], detector: (id: string) => boolean): number[] =>
  ranges
    .map((r) => r.split("-").map(Number))
    .map(([from, to]) =>
      range(from, to + 1)
        .map(String)
        .filter(detector)
    )
    .flat()
    .map(Number)

const input = await readFile("input.txt", { encoding: "utf8" })
const ranges = input.split("\n").join("").split(",")
const fakeIds1 = detectFakeIds(ranges, isFakeId({ repeats: 2 }))
const fakeIds2 = detectFakeIds(ranges, isFakeId())

console.log(`🛒 Fake product IDs found... summing them up to ${sum(fakeIds1)}`)
console.log(`🛒 Even more fake product IDs found... summing them up to ${sum(fakeIds2)}`)
