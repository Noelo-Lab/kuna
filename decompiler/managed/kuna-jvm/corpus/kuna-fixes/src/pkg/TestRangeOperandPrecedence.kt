package pkg

fun allDigits(s: String): Boolean {
    val length = s.length
    if (length == 0) return false
    for (index in (if (s[0] == '-') 1 else 0) until length) {
        if (s[index] !in '0'..'9') return false
    }
    return true
}

fun sumFrom(start: Int?, end: Int): Int {
    var total = 0
    for (k in (start ?: 0)..end) {
        total += k
    }
    return total
}

fun countDown(from: Int, extra: Int?): Int {
    var total = 0
    for (k in from downTo (extra ?: 1)) {
        total += k
    }
    return total
}
