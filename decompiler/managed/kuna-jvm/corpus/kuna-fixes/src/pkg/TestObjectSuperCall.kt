package pkg

class TestObjectSuperCall(val q: Int) {
    abstract class Base(val n: Int, val tag: String) {
        abstract fun get(): Int
    }

    open class Plain {
        open fun get(): Int = 0
    }

    interface Marker

    fun make(k: Int, s: String): Base {
        return object : Base(k + 1, s), Marker {
            override fun get(): Int = n * 2
        }
    }

    fun plain(): Plain = object : Plain() {
        override fun get(): Int = 5
    }

    fun captured(r: Int): Base = object : Base(q, "c") {
        override fun get(): Int = r + q
    }
}
