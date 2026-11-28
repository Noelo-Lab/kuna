package pkg

class TestJavaCastSyntax(private val items: Array<Any?>) {
    fun narrow(l: Long): Int = l.toInt()
    fun toByte(d: Double): Byte = d.toInt().toByte()
    fun chr(i: Int): Char = i.toChar()
    fun minLen(byteCount: Long, limit: Int): Int = Math.min(byteCount, (8192 - limit).toLong()).toInt()
    fun ratio(a: Int, b: Long): Double = a.toDouble() / b
    fun half(x: Int): Short = (x / 2).toShort()

    @Suppress("UNCHECKED_CAST")
    fun <T> get(i: Int): T = items[i] as T

    fun <R> run(block: () -> R): R = block()
}
