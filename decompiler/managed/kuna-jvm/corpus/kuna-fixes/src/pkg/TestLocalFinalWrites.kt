package pkg

class TestLocalFinalWrites {
    fun followUps(limit: Int, step: Int): Int {
        var count = 0
        while (true) {
            if (step > 3) {
                if (++count > limit) {
                    throw IllegalStateException("too many: $count")
                }
            }
            if (count * step == 42) return count
        }
    }

    fun charIndex(bytes: ByteArray, codePoints: Int): Int {
        var chars = 0
        var j = 0
        var i = 0
        while (i < bytes.size) {
            if (bytes[i] >= 0) {
                if (j++ == codePoints) return chars
                chars++
            }
            i++
        }
        return chars
    }

    fun lastSegment(path: String, start: Int): Int {
        var offset = start
        var segmentEnd: Int
        var n = 0
        do {
            segmentEnd = offset
            while (segmentEnd < path.length && path[segmentEnd] != '/') segmentEnd++
            n += segmentEnd - offset
            offset = segmentEnd + 1
        } while (segmentEnd < path.length)
        return n
    }
}
