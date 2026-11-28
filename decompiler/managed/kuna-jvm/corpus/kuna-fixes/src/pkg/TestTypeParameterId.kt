package pkg

class TestTypeParameterId<K, V : Comparable<V>>(private val keys: Array<K>, private var best: V) {
    fun key(i: Int): K = keys[i]

    fun offer(value: V): V {
        if (value > best) best = value
        return best
    }

    fun pairs(values: List<V>): Map<K, V> = keys.zip(values).toMap()

    fun <R> mapKeys(transform: (K) -> R): List<R> = keys.map(transform)

    inner class Cursor(private var index: Int) {
        fun current(): K = keys[index]
        fun advance(step: V): Pair<K, V> {
            index++
            return keys[index] to step
        }
    }
}
