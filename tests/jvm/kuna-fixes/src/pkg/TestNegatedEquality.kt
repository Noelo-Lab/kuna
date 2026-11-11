package pkg

class TestNegatedEquality(private val name: String, private val tag: Any?) {
    fun differs(other: TestNegatedEquality): Boolean = (name == other.name) xor true
    fun stored(other: Any?): Boolean {
        val changed = (tag == other) xor true
        return changed
    }
    fun guard(a: String?, b: String?): Int {
        if ((a == b) xor true) {
            return 1
        }
        return 2
    }
    fun either(a: String?, b: String?, c: Boolean): Boolean = c or ((a == b) xor true)
    fun compareWith(a: String?, b: String?, c: Boolean): Boolean = ((a == b) xor true) == c
    fun numbers(i: Int, j: Int): Boolean = (i == j) xor true
    fun same(a: Any, b: Any): Boolean = (a === b) xor true
}
