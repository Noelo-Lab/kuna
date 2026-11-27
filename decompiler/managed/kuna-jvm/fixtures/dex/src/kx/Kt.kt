package kx

data class Point(val x: Int, val y: Int) {
    operator fun plus(o: Point) = Point(x + o.x, y + o.y)
}

sealed class Expr {
    class Num(val v: Long) : Expr()
    class Add(val l: Expr, val r: Expr) : Expr()
    object Zero : Expr()
}

fun eval(e: Expr): Long = when (e) {
    is Expr.Num -> e.v
    is Expr.Add -> eval(e.l) + eval(e.r)
    Expr.Zero -> 0L
}

fun String.shout(times: Int = 2): String = uppercase().repeat(times)

fun classify(x: Any?): String = when (x) {
    null -> "null"
    is Int -> if (x > 10) "big" else "small"
    is String -> "s:${x.length}"
    else -> "other"
}

inline fun <T> measure(block: () -> T): T = block()

object Registry {
    private val items = mutableListOf<String>()
    fun add(s: String): Int {
        items += s
        return items.size
    }
}

fun main() {
    val p = Point(1, 2) + Point(3, 4)
    println(p)
    println(p.copy(y = 9) == Point(4, 9))
    println(eval(Expr.Add(Expr.Num(5), Expr.Add(Expr.Zero, Expr.Num(37)))))
    println("hey".shout() + "x".shout(3))
    listOf(null, 3, 30, "abc", 2.0).forEach { println(classify(it)) }
    val squares = (1..6).filter { it % 2 == 0 }.map { it * it }
    println(squares.sum())
    println(measure { Registry.add("a"); Registry.add("b") })
    var acc = 0L
    for (i in 10 downTo 1 step 3) acc = acc * 31 + i
    println(acc)
    val name: String? = if (acc > 0) "named" else null
    println(name?.length ?: -1)
    val m = mapOf("k" to 1, "j" to 2)
    println(m.entries.joinToString { "${it.key}=${it.value}" })
}
