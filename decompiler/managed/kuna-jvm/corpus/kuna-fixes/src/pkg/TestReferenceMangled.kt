package pkg

@JvmInline
value class MangledTag(val v: Int)

class MangledHolder(val k: Int) {
    fun mark(t: MangledTag, x: Int): Int = t.v + x + k
}

fun boundMangled(h: MangledHolder): Int {
    val ref = h::mark
    return ref(MangledTag(1), 2)
}

fun unboundMangled(h: MangledHolder): Int {
    val ref = MangledHolder::mark
    return ref(h, MangledTag(3), 4)
}
