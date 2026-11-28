package pkg

class TestKtClassHeaderMetadata {
    interface Source<out T> {
        fun next(): T
    }

    interface Sink<in T> {
        fun put(value: T)
    }

    object Empty : Source<Nothing> {
        override fun next(): Nothing = throw NoSuchElementException("empty")
    }

    class Recorder : Sink<Any?> {
        var last: Any? = null

        override fun put(value: Any?) {
            last = value
        }
    }

    class Countdown(private var left: Int) : Iterator<Int> {
        override fun hasNext(): Boolean = left > 0

        override fun next(): Int = left--
    }

    class Sorted<T : Comparable<T>>(private val items: List<T>) {
        fun smallest(): T? = items.minOrNull()
    }

    class Ranked<T>(private val items: List<T>) where T : CharSequence, T : Comparable<T> {
        fun longest(): T? = items.maxByOrNull { it.length }
    }

    fun first(source: Source<CharSequence>): CharSequence = source.next()

    fun widen(source: Source<String>): CharSequence = first(source)

    fun feed(sink: Sink<String>, value: String) {
        sink.put(value)
    }

    fun record(recorder: Recorder, value: String): Any? {
        feed(recorder, value)
        return recorder.last
    }

    fun total(countdown: Countdown): Int {
        var sum = 0
        while (countdown.hasNext()) {
            sum += countdown.next()
        }
        return sum
    }
}
