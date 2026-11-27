package pkg

class TestKtFieldReadOnce {
    class Segment(var next: Segment?, var pos: Int)

    @Volatile private var worker: Thread? = null
    var head: Segment? = null

    fun worker(): Thread = worker ?: Thread()

    fun first(): Int {
        val h = head ?: throw IllegalStateException("empty")
        return h.pos
    }

    fun total(): Int {
        var n = 0
        head?.let { h ->
            n += h.pos
            var s = h.next!!
            while (s !== h) {
                n += s.pos
                s = s.next!!
            }
        }
        return n
    }
}
