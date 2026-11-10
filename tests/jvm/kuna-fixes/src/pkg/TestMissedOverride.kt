package pkg

class TestMissedOverride {
    interface Sink {
        fun buffer(): Sink
        fun write(value: Int): Sink
        val size: Long
        val owner: Any
    }

    class Buffer : Sink, Comparable<Buffer> {
        var total = 0L
        override fun buffer(): Buffer = this
        override fun write(value: Int): Buffer {
            total += value
            return this
        }
        override val size: Long
            get() = total
        override val owner: Buffer
            get() = this
        override fun compareTo(other: Buffer): Int = total.compareTo(other.total)
    }

    abstract class Base<T> {
        abstract fun accept(item: T): Boolean
        open fun name(): CharSequence = "base"
    }

    class Strings : Base<String>() {
        override fun accept(item: String): Boolean = item.isEmpty()
        override fun name(): String = "strings"
    }
}
