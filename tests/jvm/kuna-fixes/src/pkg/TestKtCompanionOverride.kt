package pkg

abstract class TestKtCompanionOverride {
    abstract fun nextBits(bits: Int): Int

    open fun next(): Int = nextBits(31)

    open fun next(until: Int): Int = next() % until

    abstract val size: Int

    open val label: String
        get() = "base"

    private fun step(x: Int): Int = x + 1

    fun stepped(x: Int): Int = step(x)

    interface Shape {
        fun area(): Int

        companion object : Shape {
            override fun area(): Int = 0
        }
    }

    companion object Default : TestKtCompanionOverride() {
        override fun nextBits(bits: Int): Int = bits * 7

        override fun next(): Int = 41

        override fun next(until: Int): Int = until - 1

        override val size: Int
            get() = 3

        override val label: String = "default"

        private fun step(x: Int): Int = x + 2

        fun stepTwice(x: Int): Int = step(step(x))

        @JvmStatic
        fun twice(n: Int): Int = n * 2

        @JvmStatic
        val shared: Int
            get() = 5
    }
}
