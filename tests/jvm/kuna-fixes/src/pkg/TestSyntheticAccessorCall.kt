package pkg

private var hits: Int = 0

private val PREFIX: String = "n"

private fun label(n: Int): String = PREFIX + n

private fun Int.doubled(): Int = this * 2

class TestSyntheticAccessorCall(val base: Int) {
    fun read(): String = label(base.doubled()) + PREFIX

    fun write(): Int {
        hits = hits + base
        return hits
    }
}
