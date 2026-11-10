package pkg

private val NAMES: Array<String> = arrayOf("alpha", "beta")

fun firstName(): String = NAMES[0]

class TestPropertyInitializerKey {
    val items: Array<Int> = arrayOf(1, 2, 3)

    fun name(i: Int): String = "" + items[i] + EMPTY.size

    companion object {
        private val EMPTY: Array<Any?> = arrayOfNulls(0)
    }
}
