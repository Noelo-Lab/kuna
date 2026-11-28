package pkg

class TestMethodObjectReturnType {
    @JvmOverloads
    fun <T> pick(first: T, second: T, takeSecond: Boolean = false): T = if (takeSecond) second else first

    @JvmOverloads
    fun describe(value: Any?, prefix: String = "v"): Any? = if (value == null) null else prefix + value
}
