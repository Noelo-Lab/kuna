package pkg

class TestOperatorLineBreak(
    private val firstComponentValue: String,
    private val secondComponentValue: String,
    private val thirdComponentValue: String,
    private val fourthComponentValue: String,
    private val fifthComponentValue: Int,
) {
    fun combined(): Int {
        return 31 * (31 * (31 * (31 * firstComponentValue.hashCode() + secondComponentValue.hashCode()) + thirdComponentValue.hashCode()) + fourthComponentValue.hashCode()) + fifthComponentValue
    }

    fun total(extraAmountOfSomethingThatIsLong: Int): Int {
        val totalLengthOfAllTheComponentValues = firstComponentValue.length + secondComponentValue.length + thirdComponentValue.length + fourthComponentValue.length + extraAmountOfSomethingThatIsLong
        return totalLengthOfAllTheComponentValues
    }

    fun remaining(base: Long): Long {
        val remainingAmountOfSomethingVeryLong: Long = base * 1000L - firstComponentValue.length.toLong() * 86400L - secondComponentValue.length.toLong() * 3600L - thirdComponentValue.length.toLong()
        return remainingAmountOfSomethingVeryLong
    }

    fun product(a: Int, b: Int, c: Int): Int {
        return (a + firstComponentValue.length + secondComponentValue.length + thirdComponentValue.length) * (b + fourthComponentValue.length + fifthComponentValue + c + a * b * c)
    }
}
