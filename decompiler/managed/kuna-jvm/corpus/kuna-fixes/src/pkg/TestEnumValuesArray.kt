package pkg

enum class TestEnumValuesArray(val code: Int, val label: String?) {
    FIRST(7, "one"),
    SECOND(-3, null),
    THIRD(42, "three");

    fun describe(): String = label + "=" + code
}
