package pkg

class TestDefaultArgsBridgeLookup(val base: Int) {
    @JvmName("scaled")
    fun scale(x: Int, factor: Int = base): Int = x * factor

    internal fun shift(x: Int, by: Int = x + 1): Int = x + by

    fun walk(d: Meters, steps: Int = 2): Int = d.v + steps + base

    suspend fun load(key: String = "k", times: Int = key.length): String = key + times

    @JvmName("pickInts")
    fun pick(xs: List<Int>, n: Int = 1): Int = xs.size + n

    fun pick(xs: List<String>, n: Int = 2): Int = xs.size * n

    fun <A : Sink> A.emit(text: String, sep: String = ", "): A {
        put(text)
        put(sep)
        return this
    }

    interface Sink {
        fun put(s: String)
    }

    interface Shape {
        fun area(scale: Int = 2): Int

        fun describe(name: String = "shape", width: Int = 3): String = name + width
    }

    @JvmInline
    value class Meters(val v: Int)

    companion object {
        @JvmStatic
        @JvmName("of")
        fun make(size: Int = 4, label: String = "box"): TestDefaultArgsBridgeLookup = TestDefaultArgsBridgeLookup(size + label.length)
    }
}
