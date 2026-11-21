package pkg

class TestCompoundBitwiseAssignment {
    var flags: Int = 0
    var mask: Long = 0L
    var ok: Boolean = false
    val words = IntArray(4)
    val bytes = ByteArray(4)

    fun mix(a: Int, b: Int): Long {
        flags = flags or (1 shl b)
        mask = mask or (1L shl a)
        flags = flags xor a
        mask = mask ushr 3
        flags = flags shl (a and 7)
        mask = mask and mask - 1
        ok = ok or (a > b)
        ok = ok and (flags != 0)
        words[1] = words[1] or a
        words[a and 3] = words[a and 3] xor b
        bytes[2] = (bytes[2].toInt() or a).toByte()
        return flags + mask + words[1] + bytes[2] + if (ok) 1 else 0
    }
}
