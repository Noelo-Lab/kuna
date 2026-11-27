package pkg

class TestMappedJavaMember {
    fun lengths(s: String, cs: CharSequence, sb: StringBuilder): Int = s.length + cs.length + sb.length
    fun firstChar(s: String, i: Int): Char = s[0] + s[i + 1].code
    fun counts(list: List<String>, map: Map<String, Int>, arr: IntArray, objs: Array<Any>): Int =
        list.size + map.size + map.keys.size + map.values.size + map.entries.size + arr.size + objs.size
    fun firstEntry(map: Map<String, Int>): String {
        val entry = map.entries.first()
        return entry.key + entry.value
    }
    fun describe(t: Throwable): String? = t.message ?: t.cause?.message
    fun kind(x: Any): String = x.javaClass.name
    fun total(n: Number): Long = n.toLong() + n.toInt()
    fun drop(list: MutableList<String>): String = list.removeAt(0)
    enum class Color { RED, GREEN }
    fun colorName(c: Color): String = c.name + c.ordinal
}
