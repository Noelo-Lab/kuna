package pkg

class SplitPair(val first: Int, val second: String)

class TestSplitConstructorCallKotlin(private val count: Int, private val label: String?) {
    fun build(): SplitPair = SplitPair(count, label ?: throw IllegalStateException("label == null"))

    fun orNull(): SplitPair? {
        return SplitPair(count, label ?: return null)
    }

    fun collect(keys: Array<String?>): List<SplitPair> {
        val out = ArrayList<SplitPair>()
        var i = count
        for (k in keys) {
            out.add(SplitPair(i++, k ?: continue))
        }
        return out
    }
}
