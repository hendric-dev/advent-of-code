import './bingo.dart';
import 'dart:convert';
import 'dart:io';

void main() async {
  final List<List<int>> lines = await readInput();
  final List<int> drawings = lines.removeAt(0);
  final List<BingoCard> bingoCards = [];
  while (!lines.isEmpty) {
    bingoCards.add(new BingoCard(lines.take(5).reduce((result, line) => result + line)));
    lines.removeRange(0, 5);
  }

  drawings.forEach((drawing) {
    bingoCards.where((card) => !card.hasWon()).forEach((card) => card.draw(drawing));
  });
  bingoCards.sort((a, b) => a.draws.compareTo(b.draws));
  BingoCard winningCard = bingoCards.first;
  BingoCard losingCard = bingoCards.last;

  print('Final Score of Winning Card: ${winningCard.score() * winningCard.lastDrawing}');
  print('Final Score of Losing Card: ${losingCard.score() * losingCard.lastDrawing}');
}

Future<List<List<int>>> readInput() async {
  return await File('input.txt')
      .openRead()
      .transform(utf8.decoder)
      .transform(LineSplitter())
      .where((line) => line.length > 0)
      .map((line) =>
          line.trim().replaceAll(',', ' ').replaceAll('  ', ' ').split(' ').map((value) => int.parse(value)).toList())
      .toList();
}
