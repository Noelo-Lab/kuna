package pkg

class TestForLoopDefinitions {
   fun jumpy(a: IntArray): Int {
      var c = 0
      var i = 0
      while (i < 100) {
         c += a[i]
         i += a[i]
         i++
      }
      return c + i
   }
}
