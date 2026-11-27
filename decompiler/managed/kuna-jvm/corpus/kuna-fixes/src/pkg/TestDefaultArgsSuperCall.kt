package pkg

open class TestDefaultArgsSuperCall(val a: Int, val b: String = "b", val c: Long = 3L) {
    fun twoDefaults(x: Int): Any = object {
        inner class Sub : TestDefaultArgsSuperCall(x)

        val sub = Sub()
    }

    fun oneDefault(x: Int, y: String): Any = object {
        inner class Sub : TestDefaultArgsSuperCall(x, y)

        val sub = Sub()
    }

    fun noDefault(x: Int): Any = object {
        inner class Sub : TestDefaultArgsSuperCall(x, "x", 1L)

        val sub = Sub()
    }

    open class Overloaded(val a: Int, val b: String = "b") {
        constructor(n: Int) : this(n, "n")
    }

    fun otherConstructor(x: Int): Any = object {
        inner class Sub : Overloaded(a = x)

        val sub = Sub()
    }
}
