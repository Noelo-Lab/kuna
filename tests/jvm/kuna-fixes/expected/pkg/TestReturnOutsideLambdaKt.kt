package pkg

public var hits: Int = 0

@JvmOverloads
public fun scale(x: Int, k: Int = 3): Int {
   return x * k
}

public fun maker(k: Int): () -> Any {
   return { 
      object {
         public fun value(v: Int): Int {
            if (v > `$k`) {
               hits = hits + 1
               return v
            } else {
               return `$k`
            }
         }
      }
   }
}
