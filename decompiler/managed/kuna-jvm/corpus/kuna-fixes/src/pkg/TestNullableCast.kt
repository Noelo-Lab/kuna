package pkg

class TestNullableCast(private val names: Map<String, String>) {
    fun lengthOf(key: String): Int {
        val name = names[key] ?: return -1
        return name.length
    }

    fun describe(key: String): String = describeName(names[key])

    private fun describeName(name: String?): String = name ?: "none"
}
