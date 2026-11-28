package pkg

import kotlin.jvm.internal.Intrinsics

class TestKtLegacyFunctionChecks(private val prefix: String) {
    interface Formatter {
        fun format(value: String, width: Int): String
    }

    class Padded : Formatter {
        override fun format(value: String, width: Int): String {
            var text = value
            while (text.length < width) {
                text = " " + text
            }
            return text
        }
    }

    fun join(first: String, second: String?, rest: List<String>): String {
        return prefix + first + second + rest.size
    }

    fun pick(a: String, b: String): String {
        if (a.isEmpty()) {
            return b
        }
        return a
    }

    inline fun <T> measure(label: String, block: () -> T): T {
        val result = block()
        record(label)
        return result
    }

    fun record(label: String) {
        count += label.length
    }

    var count: Int = 0

    fun String.shout(times: Int): String {
        var res = this
        for (i in 0 until times) {
            res = res + "!"
        }
        return res
    }

    fun late(first: String, second: Any?): Int {
        val n = first.length
        Intrinsics.checkNotNullParameter(second, "second")
        return n
    }

    private fun hidden(text: String): Int {
        return text.length
    }

    fun total(texts: Array<String>): Int {
        var sum = 0
        for (t in texts) {
            sum += hidden(t)
        }
        return sum
    }
}
