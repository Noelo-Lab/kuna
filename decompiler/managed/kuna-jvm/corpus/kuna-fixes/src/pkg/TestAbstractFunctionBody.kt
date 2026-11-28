package pkg

abstract class TestAbstractFunctionBody {
    abstract fun dispose()
    abstract fun size(): Int
    protected abstract fun name(prefix: String): String
    open fun describe(): String = name("x") + size()
    external fun nativeHash(seed: Long): Int

    interface Source {
        fun read(into: IntArray, count: Long): Long
        fun close()
    }

    abstract class Partial : Source {
        abstract override fun close()
        override fun read(into: IntArray, count: Long): Long = count
    }
}
