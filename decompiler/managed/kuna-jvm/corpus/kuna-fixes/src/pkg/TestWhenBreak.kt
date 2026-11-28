package pkg

class TestWhenBreak {
    fun unitByName(name: String): Int = when (name) {
        "d" -> 1
        "h" -> 2
        "us" -> 3
        else -> throw IllegalArgumentException("unknown $name")
    }

    fun padding(code: Int, index: Int, end: Int): Int {
        val result: Int
        when (code) {
            -6 -> {
                check(index)
                result = index + 1
            }
            -4 -> {
                check(index)
                val second = index + 2
                if (second >= end) {
                    throw IllegalArgumentException("missing pad at $second")
                }
                result = second + 1
            }
            -2 -> result = index + 1
            else -> throw IllegalStateException("unreachable")
        }
        return result
    }

    fun repeatText(text: String, n: Int): String {
        val result: String
        when (n) {
            0 -> result = ""
            1 -> result = text
            else -> {
                when (text.length) {
                    0 -> result = ""
                    1 -> {
                        val c = text[0]
                        val chars = CharArray(n)
                        for (i in 0 until n) {
                            chars[i] = c
                        }
                        result = String(chars)
                    }
                    else -> {
                        val sb = StringBuilder()
                        for (i in 1..n) {
                            sb.append(text)
                        }
                        result = sb.toString()
                    }
                }
            }
        }
        return result
    }

    private fun check(index: Int) {
        if (index < 0) {
            throw IllegalArgumentException("negative")
        }
    }

    private fun work(code: Int): Int = code * 3
}
