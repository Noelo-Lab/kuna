package pkg

import pkg.lib.LIMIT
import pkg.lib.counter
import pkg.lib.greet
import pkg.lib.lonely
import pkg.lib.names
import pkg.lib.twice

class TestFacadeCall(val base: Int) {
    fun plain(): Int = twice(base) + lonely()

    fun property(): Int {
        counter = counter + base
        return counter + LIMIT + names.size
    }

    fun defaults(): String = greet() + greet("y")

    fun shadowedByMember(): Int = pkg.lib.twice(base) + twice()

    fun twice(): Int = base * 4

    fun shadowedByLocal(): Int {
        val counter = twice()
        counter.hashCode()
        return counter * counter + pkg.lib.counter
    }
}
