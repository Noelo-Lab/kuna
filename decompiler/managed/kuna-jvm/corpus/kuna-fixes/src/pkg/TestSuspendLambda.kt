package pkg

interface LambdaSource {
    suspend fun next(): Int
    suspend fun text(i: Int): String
}

fun interface LambdaSink<T> {
    suspend fun emit(value: T)
}

enum class LambdaMode { ONE, TEN }

fun twice(src: LambdaSource): suspend () -> Int = { src.next() + src.next() }

fun plain(x: Int): suspend () -> Int = { x + 1 }

fun sumOf(src: LambdaSource, n: Int): suspend () -> Int = {
    var s = 0
    var i = 0
    while (i < n) {
        s += src.next()
        i++
    }
    s
}

fun lengthPlus(src: LambdaSource): suspend (String) -> Int = { s -> s.length + src.next() }

fun pump(src: LambdaSource): suspend LambdaSink<Int>.(Int) -> Unit = { n ->
    var i = 0
    while (i < n) {
        emit(src.next())
        i++
    }
}

fun guarded(src: LambdaSource): suspend () -> String = {
    try {
        src.text(src.next())
    } catch (e: IllegalStateException) {
        "caught " + e.message
    }
}

fun scaled(mode: LambdaMode, src: LambdaSource): suspend () -> Int = {
    when (mode) {
        LambdaMode.ONE -> src.next()
        LambdaMode.TEN -> src.next() * 10
    }
}

fun firstPositive(src: LambdaSource): suspend () -> Int = search@{
    var k = 0
    while (k < 5) {
        val v = src.next()
        if (v > 0) return@search v
        k++
    }
    -1
}

suspend fun runTwice(block: suspend () -> Unit) {
    block()
    block()
}

fun nested(src: LambdaSource): suspend LambdaSink<Int>.() -> Unit = { runTwice { emit(src.next()) } }

private suspend fun helper(src: LambdaSource): Int = src.next() + 1

fun viaHelper(src: LambdaSource): suspend () -> Int = { helper(src) * 2 }
