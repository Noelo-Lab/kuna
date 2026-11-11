package com.ex.kt

class Counter(start: Int) {
    var count: Int = start
        private set

    var label: String = "counter"

    fun bump(by: Int = 1): Int {
        count += by
        return count
    }

    companion object {
        fun of(text: String): Counter = Counter(text.length)
    }
}
