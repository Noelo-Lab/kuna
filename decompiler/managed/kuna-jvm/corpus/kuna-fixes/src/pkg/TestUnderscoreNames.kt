package pkg

class TestUnderscoreNames {
   fun twice(`_`: Int): Int {
      return `_` * 2
   }

   fun sum(xs: IntArray): Int {
      var `__` = 0
      for (`_` in xs) {
         `__` += `_`
      }
      return `__`
   }
}
