package pkg

class TestKtInlineEvaluationOrder {
    interface Src {
        fun next(): Int
        fun take(a: Int, b: Int)
        fun take1(a: Int)
        fun enter(): Int
        fun leave(token: Int)
        fun run()
    }

    fun callOrder(src: Src) {
        val first = src.next()
        src.take(src.next(), first)
    }

    fun conditional(src: Src, k: Int) {
        val v = src.next()
        src.take1(if (k > 0) v else 0)
    }

    fun shortCircuit(src: Src, enabled: Boolean) {
        val v = src.next()
        if (enabled && v < 0) {
            src.run()
        }
    }

    fun element(a: IntArray, src: Src) {
        val x = a[0]
        src.take(src.next(), x)
    }

    fun scoped(src: Src) {
        val token = src.enter()
        try {
            src.run()
        } finally {
            src.leave(token)
        }
    }

    fun firstUse(src: Src, a: IntArray) {
        val v = src.next()
        src.take(v, a[0])
    }
}
