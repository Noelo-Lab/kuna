package pkg

class TestKtObjectMonitorCall {
    private val lock = Any()
    private var ready = false

    fun await() {
        synchronized(lock) {
            while (!ready) {
                (lock as Object).wait()
            }
        }
    }

    fun signal() {
        synchronized(lock) {
            ready = true
            (lock as Object).notifyAll()
        }
    }

    fun awaitFor(ms: Long, ns: Int) {
        synchronized(this) {
            if (ns == 0) {
                (this as Object).wait(ms)
            } else {
                (this as Object).wait(ms, ns)
            }
        }
    }

    fun signalOne() {
        synchronized(this) {
            (this as Object).notify()
        }
    }
}
