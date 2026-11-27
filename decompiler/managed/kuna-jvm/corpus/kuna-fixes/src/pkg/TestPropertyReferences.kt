package pkg

class Box(val size: Int) {
    var count: Int = 0
}

val answer: Int = 42

val Box.doubled: Int
    get() = size * 2

fun bound(box: Box): String = "${box::size}"

fun boundMutable(box: Box): Int {
    val ref = box::count
    return ref.get()
}

fun member(box: Box): Int {
    val ref = Box::size
    return ref.get(box)
}

fun mutableMember(box: Box) {
    val ref = Box::count
    ref.set(box, 1)
}

fun topLevel(): Int {
    val ref = ::answer
    return ref.get()
}

fun extension(box: Box): Int {
    val ref = Box::doubled
    return ref.get(box)
}
