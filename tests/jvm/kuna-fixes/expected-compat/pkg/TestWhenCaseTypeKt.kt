package pkg

public fun pathKind(name: String): Int {
   when (name.hashCode()) {
      46 -> {
         if (name.equals(".")) {
            return 1
         }
      }
      '뉯' -> {
         if (name.equals("../")) {
            return 2
         }
      }
      '랢' -> {
         if (name.equals("-x-")) {
            return 3
         }
      }
      '쥨' -> {
         if (name.equals("2nd")) {
            return 4
         }
      }
      else -> {}
   }

   return 0
}
