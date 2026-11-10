package pkg

class SplitEntry(val index: Int, val name: String)

class SplitFormat(val upper: Boolean, val bytes: SplitPart, val number: SplitPart)

class SplitPart(val width: Int) {
    class Builder(var width: Int) {
        fun build(): SplitPart? = if (width < 0) null else SplitPart(width)
    }
}

class TestSplitConstructorCall(private val names: Iterator<String>) {
    var name: String? = null
    var domain: String? = null
    var index = 0
    var upper = false
    var bytes: SplitPart.Builder? = null
    var number: SplitPart.Builder? = null

    fun build(): SplitEntry = SplitEntry(index, name ?: throw IllegalStateException("name == null"))

    fun both(): SplitEntry = SplitEntry(
        (domain ?: throw IllegalStateException("domain == null")).length,
        name ?: throw IllegalStateException("name == null")
    )

    fun next(): SplitEntry = SplitEntry(checkIndex(index++), names.next())

    fun format(): SplitFormat = SplitFormat(upper, bytes?.build() ?: SplitPart(1), number?.build() ?: SplitPart(2))

    fun orNull(): SplitEntry? {
        return SplitEntry(index, name ?: return null)
    }

    fun guarded(): String {
        try {
            val e = SplitEntry(index, name ?: throw IllegalArgumentException("no name"))
            return e.name
        } catch (e: IllegalArgumentException) {
            return "caught " + e.message
        }
    }

    fun collect(keys: Array<String?>): List<SplitEntry> {
        val out = ArrayList<SplitEntry>()
        for (k in keys) {
            out.add(SplitEntry(index++, k ?: continue))
        }
        return out
    }

    private inline fun checkIndex(i: Int): Int {
        if (i < 0) throw ArithmeticException("Index overflow has happened.")
        return i
    }
}
