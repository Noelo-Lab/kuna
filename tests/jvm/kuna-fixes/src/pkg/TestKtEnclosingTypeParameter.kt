package pkg

class TestKtEnclosingTypeParameter<K>(private val key: K) {
    interface Source<out T> {
        fun next(): T
    }

    interface Pump<out T> {
        fun drainTo(sink: MutableCollection<in T>, count: Int)
    }

    fun <T> constant(value: T): Source<T> = object : Source<T> {
        override fun next(): T = value
    }

    fun keys(): Source<K> = object : Source<K> {
        override fun next(): K = key
    }

    fun <T> pairs(value: T): Iterator<Pair<K, T>> = object : Iterator<Pair<K, T>> {
        var left = 2

        override fun hasNext(): Boolean = left > 0

        override fun next(): Pair<K, T> {
            left--
            return key to value
        }
    }

    fun <T> pumpOf(value: T): Pump<T> = object : Pump<T> {
        override fun drainTo(sink: MutableCollection<in T>, count: Int) {
            var target = sink
            var n = 0
            while (n < count) {
                target.add(value)
                if (target.size > 8) {
                    target = ArrayList()
                }
                n++
            }
        }
    }

    inline fun <T> computed(crossinline make: () -> T): Source<T> = object : Source<T> {
        override fun next(): T = make()
    }

    fun <T> later(value: T): Source<T> = computed { value }

    fun name(): Source<String> = computed { "name" }
}
