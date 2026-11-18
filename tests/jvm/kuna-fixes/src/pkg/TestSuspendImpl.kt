package pkg

interface ImplSource {
    suspend fun next(): Int
}

open class TestSuspendImpl(val src: ImplSource) {
    open suspend fun sum(n: Int): Int {
        var s = 0
        var i = 0
        while (i < n) {
            s += src.next()
            i++
        }
        return s
    }

    open suspend fun first(): Int = src.next() + offset()

    protected open fun offset(): Int = 0
}

class TestSuspendImplDoubler(src: ImplSource) : TestSuspendImpl(src) {
    override suspend fun sum(n: Int): Int = super.sum(n) * 2

    override fun offset(): Int = 100
}
