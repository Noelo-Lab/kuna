package pkg

import kotlin.coroutines.ContinuationInterceptor
import kotlin.coroutines.coroutineContext
import kotlin.coroutines.resume
import kotlin.coroutines.suspendCoroutine

interface StateSource {
    suspend fun next(): Int
    suspend fun text(i: Int): String
}

suspend fun combine(src: StateSource, scale: Long, flag: Boolean): String {
    val a = src.next()
    val d = a * 1.5
    val b = src.next()
    val s = src.text(a + b)
    return if (flag) "$s:${a + b}:$scale:$d" else s
}

suspend fun firstAbove(src: StateSource, limit: Int, tries: Int): Int? {
    var left = tries
    while (left > 0) {
        val v = src.next()
        if (v > limit) {
            return v
        }
        left--
    }
    return null
}

suspend fun guarded(src: StateSource): Int {
    return try {
        src.next() + src.next()
    } catch (e: IllegalStateException) {
        -1
    }
}

suspend fun closing(src: StateSource, log: MutableList<String>): String {
    log.add("open")
    try {
        val t = src.text(src.next())
        log.add(t)
        return t
    } finally {
        log.add("close")
    }
}

suspend fun StateSource.sumOf(n: Int): Int {
    var t = 0
    var i = 0
    while (i < n) {
        t += next()
        i++
    }
    return t
}

suspend inline fun StateSource.pair(): Int = next() * 10 + next()

suspend fun pairs(src: StateSource): Int = src.pair() + src.pair()

suspend fun mapped(src: StateSource, f: suspend (Int) -> String): String {
    val first = f(src.next())
    return first + f(src.next())
}

suspend fun intercepted(src: StateSource): String {
    val n = src.next()
    return "$n:" + (coroutineContext[ContinuationInterceptor] == null)
}

suspend fun handOff(x: Int): Int {
    val y = suspendCoroutine<Int> { it.resume(x + 1) }
    return y * 2
}
