package pkg

class TestKtInlineLockLocal(private val probe: Probe) {
    interface Probe {
        fun held(o: Any): Boolean
    }

    @Volatile
    @JvmField
    var lock: Any = Any()

    @JvmField
    var slots: Array<Any> = arrayOf(Any(), Any())

    private val fixed: Any = Any()
    var count = 0

    fun field(): Boolean {
        val l = lock
        synchronized(l) {
            count++
            return probe.held(l)
        }
    }

    fun element(i: Int): Boolean {
        val e = slots[i]
        synchronized(e) {
            count++
            return probe.held(e)
        }
    }

    fun usedAfter(): Any {
        val l = lock
        synchronized(l) {
            count++
        }
        return l
    }

    fun finalField(): Boolean {
        val l = fixed
        synchronized(l) {
            count++
            return probe.held(l)
        }
    }

    fun single(): Int {
        val l = lock
        synchronized(l) {
            return ++count
        }
    }
}
