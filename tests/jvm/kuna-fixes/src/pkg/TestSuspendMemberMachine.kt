package pkg

interface MemberSource {
    suspend fun next(): Int
    suspend fun text(i: Int): String
}

class TestSuspendMemberMachine(private val src: MemberSource) {
    var total = 0L

    suspend fun addAll(n: Int): Long {
        for (i in 0 until n) {
            total += src.next()
        }
        return total
    }

    suspend fun label(prefix: String): String {
        val a = src.next()
        val t = src.text(a)
        return prefix + t + total
    }
}
