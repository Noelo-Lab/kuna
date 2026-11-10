package pkg

val GREETING: String = "hello".uppercase()
var counter: Int = 3
    private set

fun bumpCounter(): Int = ++counter

open class TestMisplacedModality {
    var size: Long = 0L
        get() = field + 1
        internal set(value) {
            field = value * 2
        }

    open val label: String
        get() = "label$size"
}
