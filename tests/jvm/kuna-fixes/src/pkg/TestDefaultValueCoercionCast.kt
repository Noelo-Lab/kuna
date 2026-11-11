package pkg

class TestDefaultValueCoercionCast {
    open class Base

    class Derived : Base()

    fun none(tag: CharSequence = "tag", box: Base? = null, base: Base = Derived(), arr: Array<Base> = arrayOf(Derived())): Int =
        tag.length + (if (box == null) 0 else 1) + (if (base is Derived) 2 else 3) + arr.size

    fun call(action: () -> Unit = { println("x") }, n: Int = 1) {
        action()
        println(n)
    }
}
