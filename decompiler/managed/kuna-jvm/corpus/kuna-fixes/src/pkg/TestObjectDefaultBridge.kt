package pkg

interface BridgeCombiner<T> {
    fun zero(): T
    fun twice(x: T): T = x
}

fun textBridgeCombiner(): BridgeCombiner<String> = object : BridgeCombiner<String> {
    override fun zero(): String = ""
}
