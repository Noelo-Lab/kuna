package com.ex.kt

fun total(counters: List<Counter>): Int {
    var sum = 0
    for (c in counters) {
        sum += c.count
    }
    return sum
}

fun applyTwice(f: (Int) -> Int, x: Int): Int = f(f(x))

fun quadruple(x: Int): Int = applyTwice({ it * 2 }, x)

fun describe(c: Counter): String = "count=${c.count}"
