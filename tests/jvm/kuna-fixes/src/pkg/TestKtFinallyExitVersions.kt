package pkg

class TestKtFinallyExitVersions {
    interface Src {
        fun work()
        fun use(v: Int)
        fun useAny(v: Any?)
        fun g(): Int
        fun exit(): Boolean
        fun get(): Any?
    }

    fun constant(src: Src) {
        var x = 1
        try {
            src.work()
        } finally {
            x = 2
        }
        src.use(x)
    }

    fun computed(src: Src, k: Int) {
        var x = k
        try {
            src.work()
        } finally {
            x = k + 7
        }
        src.use(x)
    }

    fun conditional(src: Src, k: Int) {
        var x = k
        try {
            src.work()
        } finally {
            if (src.g() > 0) {
                x = 3
            }
        }
        src.use(x)
    }

    fun withCatch(src: Src) {
        var x = 1
        try {
            src.work()
        } catch (e: IllegalStateException) {
            x = 4
        } finally {
            x += src.g()
        }
        src.use(x)
    }

    fun nested(src: Src) {
        var x = 1
        try {
            try {
                src.work()
            } finally {
                x = 5
            }
            src.use(x)
        } finally {
            x = 6
        }
        src.use(x)
    }

    fun loop(src: Src) {
        var x = 0
        while (src.g() > 0) {
            try {
                if (src.g() > 5) break
                src.work()
            } finally {
                x = src.g()
            }
        }
        src.use(x)
    }

    fun finallyLocal(src: Src): Any? {
        var flag = false
        try {
            val r = src.get()
            flag = true
            return r
        } finally {
            val timed = src.exit()
            if (timed && flag) throw IllegalStateException()
        }
    }
}
