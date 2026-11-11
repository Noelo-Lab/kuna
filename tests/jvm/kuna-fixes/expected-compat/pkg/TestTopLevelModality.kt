@file:JvmName(name = "TestTopLevelModality")

package pkg

public final val GREETING: String = "hello"

public final var counter: Int = 3
   public final get() {
      return counter + 1
   }

   private set

public final var limit: Long = 8L
   public final set(value) {
      limit = if (value < 0L) 0L else value
   }


public fun bumpCounter(): Int {
   counter = counter + 1
   return counter
}
