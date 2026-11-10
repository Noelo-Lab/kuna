package pkg

class TestReferenceInequality {
    class Key(val id: Int) {
        override fun equals(other: Any?): Boolean = other is Key && other.id == id
        override fun hashCode(): Int = id
    }

    fun same(a: Key, b: Key): Boolean = a === b
    fun notSame(a: Key, b: Key): Boolean = a !== b
    fun notEqual(a: Key, b: Key): Boolean = a != b
    fun equal(a: Key, b: Key): Boolean = a == b
    fun branchNotSame(a: Key, b: Key): Int {
        if (a !== b) {
            return 1
        }
        return 2
    }
    fun branchSame(a: Key, b: Key): Int {
        if (a === b) {
            return 1
        }
        return 2
    }
    fun branchNotEqual(a: Key, b: Key): Int {
        if (a != b) {
            return 1
        }
        return 2
    }
    fun loopNotSame(a: Key, b: Key): Int {
        var n = 0
        var x: Key? = a
        while (x !== b) {
            n++
            x = if (n > 3) b else a
        }
        return n
    }
}
