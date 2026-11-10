package pkg

class TestSourceLocalsKt(private val id: Int, private val names: Array<String>) {
    override fun hashCode(): Int {
        var result = id
        result = 31 * result + names.size
        return result
    }

    fun hex(b: Int): String {
        val result = CharArray(2)
        result[0] = DIGITS[b shr 2 and 3]
        result[1] = DIGITS[b and 3]
        return String(result)
    }

    fun pick(c: Boolean, a: String, b: String): String {
        val res = if (c) a else b
        return res
    }

    companion object {
        private val DIGITS = charArrayOf('0', '1', '2', '3')
    }
}
