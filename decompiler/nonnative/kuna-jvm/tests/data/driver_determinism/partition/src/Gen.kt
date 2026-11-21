package pkg

abstract class Gen {
    abstract fun next(until: Int): Int

    companion object Default : Gen() {
        override fun next(until: Int): Int = until - 1
    }
}

fun Gen.next(range: IntRange): Int = next(range.last)
