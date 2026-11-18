package pkg

class TestCompoundBitwiseInfixOperand(val bits: Int) {
    infix fun mask(b: Int): Int = bits and b

    fun mix(arr: IntArray, i: Int, b: Int) {
        arr[i] = arr[i] xor (this mask b)
    }

    fun check(flags: BooleanArray, x: Any) {
        flags[0] = flags[0] and (x !is String)
    }
}
