package pkg

import pkg.lib.Counter
import pkg.lib.Meters
import pkg.lib.SubCounter

class TestJvmNameCall(val c: Counter, val s: SubCounter) {
    fun internalMember(): Int = c.bump() + s.bump()

    fun renamed(): Int = c.size() + c.get(1)

    fun inherited(): Int {
        s.closed = true
        return if (s.closed) s.size() else s.count
    }

    fun mangled(): Double = c.take(Meters(2.0)) + c.take(3.0)
}
