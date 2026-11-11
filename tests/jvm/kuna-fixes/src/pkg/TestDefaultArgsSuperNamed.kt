package pkg

open class TestDefaultArgsSuperNamed(val a: Int, val b: String = "b", val c: Long = 3L) {
    open inner class Inner(val p: Int, val q: String = "q")

    open class Overloaded(val a: Int, val b: String = "b") {
        constructor(n: Int) : this(n, "n")
    }

    fun skipsMiddle(x: Int): Any = object {
        inner class Sub : TestDefaultArgsSuperNamed(x, c = 9L)

        val sub = Sub()
    }

    fun innerSuper(z: Int): Any = object {
        inner class Sub : Inner(z)

        val sub = Sub()
    }

    fun picksPrimary(x: Int): Any = object {
        inner class Sub : Overloaded(a = x)

        val sub = Sub()
    }

    fun picksSecondary(x: Int): Any = object {
        inner class Sub : Overloaded(x)

        val sub = Sub()
    }
}
