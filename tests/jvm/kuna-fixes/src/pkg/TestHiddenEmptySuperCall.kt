package pkg

class TestHiddenEmptySuperCall {
    open class Base {
        open fun size(): Int = 0
    }

    open class Gen<T> {
        val items = ArrayList<T>()
    }

    class Plain(val label: String) : Base() {
        override fun size(): Int = label.length
    }

    class Typed(val n: Int) : Gen<String>()

    object Single : Base()

    class Holder {
        companion object : Base()
    }

    class OnlySecondary : Base {
        constructor(s: String) {
            remember(s)
        }

        constructor(n: Int)

        fun remember(s: String): Int = s.length + size()
    }

    open inner class InnerBase

    inner class InnerSub(val n: Int) : InnerBase()
}
