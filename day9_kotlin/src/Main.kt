import java.io.File

fun main() {
  val data =
      File("C:\\Users\\Fabian\\Code\\AdventOfCode\\2023\\day9_kotlin\\input").readLines().filter {
        it.isNotEmpty()
      }

  var result1 = 0;
  var result2 = 0;

  for (line in data) {
    val sequence = line.split(" ")
    val diff = mutableListOf<MutableList<Int>>()
    diff.add(mutableListOf())
    sequence.iterator().forEach { diff.first().add(it.toInt()) }
    while (diff.last().any { it != 0 }) {
      diff.add(mutableListOf())
      for (i in 0..<(diff.first().size - diff.size + 1)) {
        diff.last().add(diff[diff.size - 2][i + 1] - diff[diff.size - 2][i])
      }
    }
    for (i in (1 ..< diff.size).reversed()) {
      diff[i-1].add(diff[i-1].last() + diff[i].last())
    }
    result1 += diff[0].last()
    for (i in (1 ..< diff.size).reversed()) {
      diff[i-1].add(0, diff[i-1].first() - diff[i].first())
    }
    result2 += diff[0].first()
  }
  println(result1)
  println(result2)
}
