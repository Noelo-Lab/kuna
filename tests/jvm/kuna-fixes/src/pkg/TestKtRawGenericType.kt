package pkg

class TestKtRawGenericType(private val names: MutableList<String>) {
    fun findMatching(prefix: String): List<String> {
        val result = mutableListOf<String>()
        val i = names.iterator()
        while (i.hasNext()) {
            val name = i.next()
            if (name.length > prefix.length) {
                result.add(name)
            }
        }
        return result
    }

    fun totalLength(prefix: String): Int {
        val matches = findMatching(prefix)
        var total = 0
        val i = matches.iterator()
        while (i.hasNext()) {
            total += i.next().length
        }
        return total + matches.size
    }

    fun prune(limit: Int) {
        val i = names.iterator()
        while (i.hasNext()) {
            if (i.next().length > limit) {
                i.remove()
            }
        }
    }

    fun noneEmpty(values: Iterable<String>): Boolean {
        return values.none { it.isEmpty() }
    }

    fun depth(value: Any): Int {
        var c: Class<*>? = value.javaClass
        var n = 0
        while (c != null) {
            n++
            c = c.superclass
        }
        return n
    }

    fun lastSeen(values: List<String>): String? {
        var last: String? = null
        val visit: (String) -> Unit = { last = it }
        for (v in values) {
            visit(v)
        }
        return last
    }
}
