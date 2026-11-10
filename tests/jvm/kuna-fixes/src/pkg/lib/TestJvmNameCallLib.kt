package pkg.lib

@JvmInline
value class Meters(val value: Double)

open class Counter(var count: Int) {
    internal fun bump(): Int = ++count

    @JvmName("sizeOf")
    fun size(): Int = count

    @JvmName("getByte")
    operator fun get(i: Int): Int = count + i

    var closed: Boolean = false

    fun take(m: Meters): Double = m.value

    fun take(d: Double): Double = -d
}

class SubCounter : Counter(5)
