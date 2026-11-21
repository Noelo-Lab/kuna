package pkg

class TestKtEscapedClassName {
    object `-Legacy` {
        fun id(): Int = 7
    }

    class `in`(val v: Int)

    val legacy: `-Legacy` = `-Legacy`

    fun make(v: Int): `in` = `in`(v)

    fun read(x: `in`): Int = x.v + `-Legacy`.id()

    fun pick(flag: Boolean): Any = if (flag) `-Legacy` else `in`(1)
}
