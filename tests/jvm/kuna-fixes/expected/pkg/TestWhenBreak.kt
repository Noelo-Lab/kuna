package pkg

public class TestWhenBreak {
   public fun unitByName(name: String): Int {
      var var10000: Int
      run label39@{
         when (name.hashCode()) {
            100 -> {
               if (!name.equals("d")) {
                  throw IllegalArgumentException("unknown $name")
               }

               var10000 = 1
            }
            104 -> {
               if (!name.equals("h")) {
                  throw IllegalArgumentException("unknown $name")
               }

               var10000 = 2
            }
            3742 -> {
               if (name.equals("us")) {
                  var10000 = 3
                  return@label39
               }

               throw IllegalArgumentException("unknown $name")
            }
            else -> throw IllegalArgumentException("unknown $name")
         }
      }

      return var10000
   }

   public fun padding(code: Int, index: Int, end: Int): Int {
      var var6: Int
      when (code) {
         -6 -> {
            this.check(index)
            var6 = index + 1
         }
         -5, -3 -> throw IllegalStateException("unreachable")
         -4 -> {
            this.check(index)
            val second: Int = index + 2
            if (second >= end) {
               throw IllegalArgumentException("missing pad at $second")
            }

            var6 = second + 1
         }
         -2 -> var6 = index + 1
         else -> throw IllegalStateException("unreachable")
      }

      return var6
   }

   public fun repeatText(text: String, n: Int): String {
      var var7: String
      when (n) {
         0 -> var7 = ""
         1 -> var7 = text
         else -> {
            when (text.length) {
               0 -> var7 = ""
               1 -> {
                  val var8: Char = text[0]
                  val var9: CharArray = CharArray(n)

                  repeat(n) { ix ->
                     var9[ix] = var8
                  }

                  var7 = String(var9)
               }
               else -> {
                  val sb: StringBuilder = StringBuilder()
                  var i: Int = 1
                  if (i <= n) {
                     while (true) {
                        sb.append(text)
                        if (i == n) {
                           break
                        }

                        i++
                     }
                  }

                  var7 = sb.toString()
               }
            }
         }
      }

      return var7
   }

   private fun check(index: Int) {
      if (index < 0) {
         throw IllegalArgumentException("negative")
      }
   }

   private fun work(code: Int): Int {
      return code * 3
   }
}
