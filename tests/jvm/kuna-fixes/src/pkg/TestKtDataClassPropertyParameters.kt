package pkg

class TestKtDataClassPropertyParameters {
    data class Header(val name: String, var value: Int) {
        val size: Int = name.length + value
    }

    data class Entry<A>(private val first: A, internal val rest: List<A>) {
        fun head(): A = first
    }

    data class Defaults(val a: Int = 1, val b: String? = null)

    fun use(h: Header): Int = h.size + h.value
}
