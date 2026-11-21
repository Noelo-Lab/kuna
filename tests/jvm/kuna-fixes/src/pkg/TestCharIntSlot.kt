package pkg

fun parseSigned(s: CharSequence): Int {
    var i = 0
    val sign = when (val c = s[i]) {
        '+', '-' -> {
            ++i
            c
        }
        else -> ' '
    }
    val start = i
    var n = 0
    while (i < s.length && s[i] in '0'..'9') {
        n = n * 10 + (s[i] - '0')
        ++i
    }
    val digits = i - start
    if (digits > 3 && s[start] >= '2') {
        return -1
    }
    return if (sign == '-') -n else n
}
