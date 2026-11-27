package pkg

interface RefreshCountdown {
    fun drain(count: Long): Long
}

class TestRefreshPrintedVarNames(private val step: Long) {
    val countdown: RefreshCountdown = object : RefreshCountdown {
        override fun drain(count: Long): Long {
            var count = count
            var steps = 0L
            while (count > 0L) {
                count -= step
                steps++
            }
            return steps
        }
    }
}
