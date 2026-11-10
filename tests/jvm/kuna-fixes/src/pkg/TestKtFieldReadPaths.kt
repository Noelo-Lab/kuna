package pkg

class TestKtFieldReadPaths {
    class Holder {
        @JvmField var count: Int = 0
    }

    interface Src {
        fun work()
        fun use(v: Int)
        fun flag(): Boolean
    }

    fun inFinally(h: Holder, src: Src) {
        val v = h.count
        try {
            src.work()
        } finally {
            src.use(v)
        }
    }

    fun restoreOnFailure(h: Holder, src: Src) {
        val saved = h.count
        try {
            src.work()
        } catch (e: Throwable) {
            h.count = saved
            throw e
        }
    }

    fun afterBranch(h: Holder, src: Src, c: Boolean) {
        val v = h.count
        if (c) {
            src.work()
        }
        src.use(v)
    }

    fun afterFinally(h: Holder, src: Src, k: Int) {
        val v = h.count
        var x = k
        try {
            x++
        } finally {
            src.work()
        }
        src.use(v + x)
    }

    fun afterCondition(h: Holder, src: Src, c: Boolean) {
        val v = h.count
        val w = c && src.flag()
        src.use(if (w) v else 0)
    }
}
