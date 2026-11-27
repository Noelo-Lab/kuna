package pkg

open class TestSuperPropertyAccess {
    open val isActive: Boolean
        get() = count > 0

    open var count: Int = 0

    class Child : TestSuperPropertyAccess() {
        override val isActive: Boolean
            get() = super.isActive && count < 10

        override var count: Int
            get() = super.count + 1
            set(value) {
                super.count = value * 2
            }
    }
}
