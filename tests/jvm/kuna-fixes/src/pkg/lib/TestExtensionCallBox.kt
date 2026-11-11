package pkg.lib

class ExtBox(val v: Int) {
    fun size(): Int = v

    fun label(): String = "box"

    fun Int.scaled(): Int = this * v

    fun scaledBy(x: Int): Int = x.scaled()

    companion object {
        @JvmStatic @JvmName("parse") fun String.toBox(): ExtBox = ExtBox(length)
    }
}

object ExtTools {
    fun String.twiceOf(): String = this + this
}
