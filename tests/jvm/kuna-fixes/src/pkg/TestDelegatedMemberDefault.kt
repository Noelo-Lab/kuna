package pkg

class TestDelegatedMemberDefault {
    interface Shape {
        fun area(scale: Int = 2, label: String? = null): Int

        fun name(): String
    }

    class Wrapper(private val shape: Shape) : Shape by shape {
        fun twice(): Int = area() * 2
    }
}
