package pkg

class Broadcast(val size: Int) : ZChannel(size)

open class ZChannel(val capacity: Int, val onDrop: ((Int) -> Unit)? = null)
