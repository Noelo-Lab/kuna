package pkg

class TestDefaultValueNames(val length: Int) {
    fun String.pad(width: Int = this@TestDefaultValueNames.length): String = this + width

    fun String.cut(n: Int = length - 1, keep: Boolean = n > 0): String = if (keep) this + n else this

    fun twice(a: Int, b: Int = a * 2): Int = a + b

    interface Shape {
        fun scaled(base: Int, by: Int = base * 2): Int
    }
}
