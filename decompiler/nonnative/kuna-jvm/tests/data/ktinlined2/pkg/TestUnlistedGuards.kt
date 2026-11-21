package pkg

interface GuardSource {
    fun next(): Any?
}

inline fun guardSource(crossinline f: () -> Any?): GuardSource = object : GuardSource {
    override fun next(): Any? = f()
}

inline fun <reified T> typedSource(crossinline f: () -> T): GuardSource = guardSource { f() as? T }

inline fun guardSuspend(crossinline f: suspend () -> Int): suspend () -> Int = { f() + 1 }

class TestUnlistedGuards(private val base: Int) {
    fun later(step: Int): suspend () -> Int = guardSuspend { base + step }
}
