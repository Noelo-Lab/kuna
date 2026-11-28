package pkg

class TestKtNarrowConstantTemps {
    var seen: Int = 0

    fun note() {
        seen++
    }

    fun code(kind: Int): Int {
        return if (kind == 1) {
            note()
            20
        } else if (kind == 2) {
            note()
            10
        } else {
            30
        }
    }

    fun retry(): Int {
        var hits = 0
        repeat(3) {
            hits += it
        }
        return hits
    }

    fun limit(values: IntArray): Int {
        var sum = 0
        for (i in 0 until 10) {
            sum += values[i]
        }
        return sum
    }

    fun store(target: ByteArray, kind: Int) {
        val b: Byte = if (kind == 0) 7 else 9
        target[0] = b
    }

    fun wide(kind: Int): Int {
        return if (kind == 1) {
            note()
            300
        } else {
            256
        }
    }

    fun many(): Int {
        var total = 0
        repeat(300) {
            total += it
        }
        return total
    }

    fun keepByte(kind: Int): Byte {
        return if (kind == 1) {
            note()
            20
        } else {
            30
        }
    }

    fun shorts(kind: Int): Long {
        val s = if (kind == 0) {
            note()
            300
        } else {
            400
        }
        return s.toLong() * kind
    }
}
