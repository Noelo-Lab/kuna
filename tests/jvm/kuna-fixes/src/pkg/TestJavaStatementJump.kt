package pkg

class TestJavaStatementJump {
    private var log: Int = 0

    fun guarded(code: Int): Int {
        var r = 0
        when (code) {
            1 -> {
                try {
                    r = work(code)
                } finally {
                    log++
                }
            }
            2 -> r = 7
            else -> r = -1
        }
        return r + log
    }

    fun firstHit(values: IntArray): Int {
        var r = -1
        for (v in values) {
            if (v > 10) {
                try {
                    r = work(v)
                } finally {
                    log++
                }
                break
            }
        }
        return r
    }

    private fun work(code: Int): Int = code * 3
}
