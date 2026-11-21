package pkg

class TestJvmOverloads(private val base: Int) {
    @JvmOverloads
    fun scaled(x: Int, factor: Int = 2, offset: Int = base): Int = x * factor + offset

    companion object {
        @JvmOverloads
        fun join(a: String, sep: String = ",", b: String = "z"): String = a + sep + b
    }
}
