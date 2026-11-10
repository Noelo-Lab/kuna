package pkg

class TestGeneratedConstructors(val size: Int = 4, val name: String? = null) {
    class Overloads @JvmOverloads constructor(val a: Int, val b: Long = 2L, val c: Boolean = false)

    class Secondary(val a: Int, val b: Int = 1) {
        constructor(s: String) : this(s.length)
    }

    fun make(): Any = TestGeneratedConstructors()

    fun overloads(): Any = Overloads(1)
}
