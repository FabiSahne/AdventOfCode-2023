import java.io.File

fun main() {
    val data =
        File("C:\\Users\\Fabian\\Code\\AdventOfCode\\2023\\day15_kotlin\\input")
            .readText().filter { !it.isWhitespace() }
            .split(',')

    //val data = listOf("rn=1","cm-","qp=3","cm=2","qp-","pc=4","ot=9","ab=5","pc-","pc=6","ot=7")

    var result = 0

    for (str in data) {
        var current = 0
        for (cha in str) {
            current = ((current + cha.code) * 17) % 256
        }
        result += current
    }

    println(result)

    val boxes = hashMapOf<Int, MutableList<String>>()

    for (str in data) {
        val chaIter = str.iterator()
        var cha = chaIter.next()
        var label = ""
        while (cha.isLetter()) {
            label += cha
            cha = chaIter.next()
        }
        if (cha == '-') {
            boxes.values.find { it.contains(str) }
        }

    }

}