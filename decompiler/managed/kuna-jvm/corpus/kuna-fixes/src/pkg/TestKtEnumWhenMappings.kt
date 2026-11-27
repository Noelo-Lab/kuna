package pkg

class TestKtEnumWhenMappings {
   enum class Color { RED, GREEN, BLUE }

   enum class Size { SMALL, MEDIUM, LARGE }

   fun name(c: Color): String = when (c) {
      Color.RED -> "red"
      Color.GREEN -> "green"
      Color.BLUE -> "blue"
   }

   fun reordered(c: Color): Int = when (c) {
      Color.BLUE -> 10
      Color.RED -> 20
      else -> 30
   }

   fun grouped(c: Color, s: Size): Int {
      var r = 1
      when (c) {
         Color.RED, Color.BLUE -> r += 2
         else -> r += 3
      }
      when (s) {
         Size.LARGE -> r *= 5
         Size.SMALL -> r *= 7
         else -> {}
      }
      return r
   }

   fun nullable(c: Color?): String = when (c) {
      null -> "none"
      Color.GREEN -> "go"
      else -> "stop"
   }

   fun nullableExhaustive(s: Size?): Int = when (s) {
      Size.SMALL -> 1
      Size.MEDIUM -> 2
      Size.LARGE -> 3
      null -> 0
   }
}
