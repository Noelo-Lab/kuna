package pkg

class TestKtFunctionParameterName(val id: Int) {
    fun plus(other: TestKtFunctionParameterName, extra: Int): Int = id + other.id + extra

    override fun equals(other: Any?): Boolean = other is TestKtFunctionParameterName && other.id == id

    override fun hashCode(): Int = id

    fun String.tagged(prefix: String): String = prefix + this + id

    fun shift(by: Int): Int {
        var v = by
        v += id
        return v
    }
}
