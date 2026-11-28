package pkg

class TestSwitchHeadIndent {
    var seen = 0

    fun note(v: Int): Int {
        seen += v
        return seen
    }

    fun pick(s: String, n: Int): Int {
        val a = note(n * 2)
        val b = note(s.length + a)
        return when (n) {
            1 -> a
            2 -> b
            3 -> a - b
            else -> a + b
        }
    }

    fun name(s: String): Int {
        val k = note(s.length)
        return when (s) {
            "one" -> 1 + k
            "two" -> 2 + k
            else -> k
        }
    }
}
