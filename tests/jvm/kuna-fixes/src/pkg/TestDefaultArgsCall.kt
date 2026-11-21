package pkg

fun join(a: Int, b: Int = 1, c: String = "c"): String = "" + a + b + c

fun pick(a: Int, b: Long = 2L): Long = a + b

fun pick(a: String, b: Long = 3L): Long = b

fun String.twice(n: Int = 2): String = if (n > 1) this + this else this

class TestDefaultArgsCall(val base: Int) {
    fun scale(x: Int = 1, y: Boolean = false, z: Char = 'z'): Int = if (y) x * base else if (z == 'q') x else x + 1

    fun same(a: Int): Int = a

    fun same(a: Int, b: Int = 0): Int = a + b

    companion object {
        fun make(size: Int = 4, label: String? = null): TestDefaultArgsCall = TestDefaultArgsCall(size)
    }

    interface Shape {
        fun area(scale: Int = 1): Int
    }

    class Box(val w: Int, val h: Int = 1, val tag: String = "box")

    fun trailing(): String = join(base)

    fun middle(): String = join(base, c = "m")

    fun all(): String = join(base, 5, "all")

    fun overloads(s: String): Long = pick(base) + pick(s)

    fun extension(s: String): String = s.twice()

    fun member(other: TestDefaultArgsCall): Int = other.scale() + scale(base) + other.scale(y = true) + scale(z = 'q', x = 3)

    fun companion(): TestDefaultArgsCall = make() 

    fun named(): TestDefaultArgsCall = make(label = "x")

    fun iface(s: Shape): Int = s.area()

    fun boxes(): Int = Box(base).h + Box(base, tag = "t").w + Box(w = 2, h = 3).h

    fun nested(): String = join(join(base).length, c = join(2, 3))
}
