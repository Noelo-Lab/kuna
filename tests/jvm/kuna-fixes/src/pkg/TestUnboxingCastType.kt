package pkg

class TestUnboxingCastType {
    fun test(p: (Int) -> Boolean, x: Int): Int = if (p(x)) 1 else 0
    fun negated(p: (Int) -> Boolean, x: Int): Int = if (!p(x)) 1 else 0
    fun both(p: (Int) -> Boolean, q: () -> Boolean, x: Int): Boolean = !p(x) && q()
    fun stored(p: (Int) -> Boolean, x: Int): Boolean {
        val ok: Boolean = p(x)
        return ok
    }
    fun char(f: () -> Char): Char = f()
    fun charPlusOne(f: () -> Char): Int = f().code + 1
    fun long(f: () -> Long): Long = f() * 2L
    fun count(items: List<String>, p: (String) -> Boolean): Int {
        var n = 0
        for (item in items) {
            if (!p(item)) n++
        }
        return n
    }
}
