package pkg

class Adder(val k: Int) {
    fun add(a: Int, b: Int): Int = a + b + k
}

fun twice(x: Int): Int = x * 2

fun bound(adder: Adder): Int {
    val ref = adder::add
    return ref(1, 2)
}

fun unbound(adder: Adder): Int {
    val ref = Adder::add
    return ref(adder, 1, 2)
}

fun constructor(): Int {
    val ref = ::Adder
    return ref(3).k
}

fun topLevel(x: Int): Int {
    val ref = ::twice
    return ref(x)
}
