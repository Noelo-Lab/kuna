package pkg

fun String.shout(times: Int): String {
    val sb = StringBuilder()
    for (i in 0 until times) sb.append(this).append('!')
    return sb.toString() + length
}

fun IntArray.total(): Long {
    var sum = 0L
    for (v in this) sum += v
    return sum
}

class TestExtensionReceiver(val base: Int) {
    fun Int.scaled(): Int = this * base + this@TestExtensionReceiver.base

    inner class Inner(val step: Int) {
        fun Int.next(): Int = this + step + base
    }

    fun run(): Int = 3.scaled() + Inner(2).run { 5.next() }

    companion object {
        fun String.twice(): String = this + this
    }
}
