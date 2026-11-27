package pkg

interface CompanionAction {
    fun act(): Int
}

class TestCompanionNamelessClass(val start: Int) {
    var count = start

    init {
        apply(object : CompanionAction {
            override fun act(): Int = count++
        })
    }

    fun apply(action: CompanionAction): Int = action.act() + LIMIT

    companion object {
        const val LIMIT = 3

        fun twice(start: Int): Int = TestCompanionNamelessClass(start).count * 2
    }
}
