package pkg

class TestKtDefaultImplsDelegate {
    interface Shape {
        val sides: Int

        fun describe(): String = "shape with $sides sides"

        fun scaled(factor: Int): Int = sides * factor

        fun greet(name: String): String = "$name has $sides sides"
    }

    interface Polygon : Shape {
        fun isTriangle(): Boolean = sides == 3
    }

    class Triangle : Polygon {
        override val sides: Int
            get() = 3
    }

    class Square : Polygon {
        override val sides: Int
            get() = 4

        override fun describe(): String = "square"
    }

    class Tagged(private val name: String) : Polygon {
        override val sides: Int
            get() = name.length

        override fun scaled(factor: Int): Int = super.scaled(factor) + 1
    }

    fun total(shapes: List<Shape>): Int {
        var sum = 0
        for (s in shapes) {
            sum += s.scaled(2)
        }
        return sum
    }
}
