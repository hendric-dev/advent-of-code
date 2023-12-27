from vent import Point, Vent

def create_vent(line):
  start, end = line.replace('\n', '').split(' -> ')
  return Vent(Point(start.split(',')[0], start.split(',')[1]), Point(end.split(',')[0], end.split(',')[1]))

def find_dangers(vents, ignore_diagonals):
  hazard_map = {}
  for connection in list(map(lambda vent: vent.connection(ignore_diagonals), vents)):
    for point in connection:
      hazard_map[f'{point.x},{point.y}'] = hazard_map.get(f'{point.x},{point.y}', 0)
      hazard_map[f'{point.x},{point.y}'] += 1
  return hazard_map

with open('input.txt') as input:
  vents = list(map(lambda line: create_vent(line), input.readlines()))
  hazard_map = find_dangers(vents, ignore_diagonals=True)
  hazard_map_with_diagonals = find_dangers(vents, ignore_diagonals=False)

  filtered = list(filter(lambda occurance: occurance >= 2, list(hazard_map.values())))
  filtered_with_diagonals = list(filter(lambda occurance: occurance >= 2, list(hazard_map_with_diagonals.values())))
  print(f'Found {len(filtered)} dangerous spots without diagonals.')
  print(f'Found {len(filtered_with_diagonals)} dangerous spots including diagonals.')
