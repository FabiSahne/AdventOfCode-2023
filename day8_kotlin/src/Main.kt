import java.io.File
import java.util.HashMap

fun main() {
    val data =
        File("C:\\Users\\Fabian\\Code\\AdventOfCode\\2023\\day8_kotlin\\input").readLines().filter { it.isNotEmpty() }

    val directions = data[0]
    val dSize = directions.length

    val nodes = HashMap<String, Pair<String, String>>()

    for (line in data) {
        if (line.contains("=")) {
            val (src, dst) = line.split(" = ")
            var (dstLeft, dstRight) = dst.split(", ")
            dstLeft = dstLeft.filter { it.isLetter() }
            dstRight = dstRight.filter { it.isLetter() }

            nodes[src] = Pair(dstLeft, dstRight)
        }
    }

    val startingNodes = nodes.filter { it.key.matches("[A-Z]{2}A".toRegex()) }

    val positions = startingNodes.keys.toMutableSet()
    var steps = 0

    while (!positions.all { it.matches("[A-Z]{2}Z".toRegex()) }) {
        val nextDir = directions[steps % dSize]
        val nextPos = nodes.filter { positions.contains(it.key) }.values
        positions.clear()
        if (nextDir == 'R') {
            nextPos.forEach { pair -> positions.add(pair.second) }
        } else if (nextDir == 'L') {

            nextPos.forEach { pair -> positions.add(pair.first) }
        }
        steps++
        if (steps % 100_000 == 0) {
            println(steps)
        }
    }

    println(steps)


}