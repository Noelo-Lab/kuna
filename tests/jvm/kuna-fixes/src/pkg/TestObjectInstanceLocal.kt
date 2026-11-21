package pkg

object TestObjectInstanceLocal {
    private val table: IntArray = IntArray(8)
    val first: Int = compute(3)

    init {
        for (i in 0 until 8) register(i)
    }

    private fun compute(x: Int): Int = x * 2

    private fun register(i: Int) {
        table[i] = i * i
    }

    fun get(i: Int): Int = table[i]
}
