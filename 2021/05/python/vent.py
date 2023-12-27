from copy import copy

class Vent:
  def __init__(self, start, end):
    self.start = start
    self.end = end

  def connection(self, ignore_diagonals=False):
    if self.is_horizontal():
      return list(map(lambda y: Point(self.start.x, y), range(min(self.start.y, self.end.y), max(self.start.y, self.end.y) + 1)))
    if self.is_vertical():
      return list(map(lambda x: Point(x, self.start.y), range(min(self.start.x, self.end.x), max(self.start.x, self.end.x) + 1)))
    return [] if ignore_diagonals else self.connection_diagonal()

  def connection_diagonal(self):
    result = []
    point = copy(self.start)
    while point.x != self.end.x and point.y != self.end.y:
      result.append(copy(point))
      point.x += 1 if self.end.x > self.start.x else -1
      point.y += 1 if self.end.y > self.start.y else -1
    result.append(copy(self.end))
    return result

  def is_horizontal(self):
    return self.start.x == self.end.x

  def is_vertical(self):
    return self.start.y == self.end.y

class Point:
  def __init__(self, x, y):
    self.x = int(x)
    self.y = int(y)
