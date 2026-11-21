package com.ex.ktfail

inline fun <T> byLength(crossinline f: (T) -> String): Comparator<T> = Comparator { a, b -> f(a).length - f(b).length }

fun sortNames(l: MutableList<String>) {
    l.sortWith(byLength { it })
}
