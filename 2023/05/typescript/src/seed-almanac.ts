export enum Category {
  Fertilizer = 'fertilizer',
  Humidity = 'humidity',
  Location = 'location',
  Light = 'light',
  Seed = 'seed',
  Soil = 'soil',
  Temperature = 'temperature',
  Water = 'water',
}

type Conversion = {
  sourceRangeStart: number
  destinationRangeStart: number
  rangeLength: number
}

type Map = {
  conversions: Conversion[]
  destination: Category
}
type Maps = Record<Category, Map>

export class SeedAlmanac {
  maps: Maps
  seeds: number[]

  constructor(input: string) {
    this.maps = this.parseMaps(input)
    this.seeds = this.parseSeeds(input)
  }

  convertSeedTo(seed: number, category: Category): number {
    const current = { category: Category.Seed, value: seed }
    while (current.category !== category) {
      const conversion = this.maps[current.category].conversions.find(
        ({ sourceRangeStart, rangeLength }) =>
          sourceRangeStart <= current.value && sourceRangeStart + rangeLength >= current.value,
      )
      if (conversion) current.value += conversion.destinationRangeStart - conversion.sourceRangeStart
      current.category = this.maps[current.category].destination
    }
    return current.value
  }

  convertLocationTo(location: number, category: Category): number {
    const current = { category: Category.Location, value: location }
    while (current.category !== category) {
      const [category, map] =
        Object.entries(this.maps).find(([_, { destination }]) => destination === current.category) ?? []
      const conversion = map?.conversions.find(
        ({ destinationRangeStart, rangeLength }) =>
          destinationRangeStart <= current.value && destinationRangeStart + rangeLength > current.value,
      )
      if (conversion) current.value -= conversion.destinationRangeStart - conversion.sourceRangeStart
      current.category = category as Category
    }
    return current.value
  }

  private parseMaps(input: string): Maps {
    return input
      .split('\n\n')
      .filter((line) => !line.includes('seeds:'))
      .reduce((maps, map) => {
        const [category, ...lines] = map.split('\n')
        const [source, destination] = category.split(' ').at(0)?.split('-to-') ?? []
        const conversions = lines.map((line) => {
          const [destinationRangeStart, sourceRangeStart, rangeLength] = line.split(' ').map(Number)
          return {
            sourceRangeStart,
            destinationRangeStart,
            rangeLength,
          }
        })
        maps[source as Category] = { destination: destination as Category, conversions }
        return maps
      }, {} as Maps)
  }

  private parseSeeds(input: string): number[] {
    const seeds = input
      .split('\n')
      .find((line) => line.includes('seeds:'))
      ?.split(':')
      ?.at(1)
      ?.trim()
      .split(' ')
      .map(Number)

    return seeds ?? []
  }

  public lowestLocationUsingSeedRanges(): number {
    let found = false
    let location = 0
    while (!found) {
      location += 1
      const seed = this.convertLocationTo(location, Category.Seed)

      for (let i = 0; i < this.seeds.length; i += 2) {
        const value = this.seeds[i]
        const range = this.seeds[i + 1]
        if (seed >= value && seed < value + range) {
          found = true
        }
      }
    }
    return location
  }
}
