package pkg

public final var hits: Int
   internal set

@JvmOverloads
public fun scale(x: Int, k: Int = 3): Int {
   return x * k
}

public fun maker(k: Int): () -> Any {
   return { 
      object {
         public fun value(v: Int): Int {
            if (v > $k) {
               hits = hits + 1
               v
            } else {
               $k
            }
         }
      }
   }
}

@JvmOverloads
fun scale(x: Int): Int {
   scale$default(x, 0, 2, null)
}
