class BingoCard {
  List<BingoField> bingoFields = [];
  int draws = 0;
  int lastDrawing = 0;

  BingoCard(List<int> values) {
    int row = 0;
    int column = 0;
    this.bingoFields = values.map<BingoField>((value) {
      BingoField field = new BingoField(row, column, value);
      column = (column + 1) % 5;
      if (column == 0) row += 1;
      return field;
    }).toList();
  }

  void draw(int value) {
    draws += 1;
    this.lastDrawing = value;
    this.bingoFields.forEach((field) {
      if (field.value == value) field.hit = true;
    });
  }

  bool hasWon() {
    return this.bingoFields.where((field) => field.hit == true).any((field) =>
        this.bingoFields.where((element) => element.row == field.row).every((element) => element.hit == true) ||
        this.bingoFields.where((element) => element.column == field.column).every((element) => element.hit == true));
  }

  int score() {
    return this
        .bingoFields
        .where((field) => field.hit == false)
        .map((field) => field.value)
        .reduce((result, value) => result + value);
  }
}

class BingoField {
  int row;
  int column;
  int value;
  bool hit = false;

  BingoField(this.row, this.column, this.value);
}
