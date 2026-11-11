package pkg

class TestMemberExtensionProperty(private val scale: Int) {
    private val Int.scaled: Int
        get() = this * scale

    fun total(values: List<Int>): Int {
        var sum = 0
        for (v in values) {
            sum += v.scaled
        }
        return sum
    }
}
