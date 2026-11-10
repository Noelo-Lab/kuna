package pkg

class TestMappedJavaType {
    fun join(parts: Array<String>): String {
        var out = ""
        for (p in parts) out += p
        return out
    }

    fun text(cs: CharSequence): String {
        val s = cs.toString()
        return s
    }

    fun boxed(values: List<Long>): Long {
        var sum = 0L
        for (v in values) sum += v
        return sum
    }

    fun catching(block: () -> Unit): String {
        try {
            block()
        } catch (t: Throwable) {
            return t.toString()
        }
        return ""
    }

    fun number(n: Number?): Number = n ?: 0
}
