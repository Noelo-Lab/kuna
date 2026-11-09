package pkg

operator fun Array<String>.iterator(): Iterator<String> = object : Iterator<String> {
    var i = 0
    override fun hasNext(): Boolean = i < size
    override fun next(): String = this@iterator[i++]
}
