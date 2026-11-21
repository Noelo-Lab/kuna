package pkg

public fun pathKind(name: String): Int {
   when (name.hashCode()) {
      46 -> {
         if (name.equals(".")) {
            return 1
         }
      }
      45679 -> {
         if (name.equals("../")) {
            return 2
         }
      }
      47010 -> {
         if (name.equals("-x-")) {
            return 3
         }
      }
      51560 -> {
         if (name.equals("2nd")) {
            return 4
         }
      }
      else -> {}
   }

   return 0
}
