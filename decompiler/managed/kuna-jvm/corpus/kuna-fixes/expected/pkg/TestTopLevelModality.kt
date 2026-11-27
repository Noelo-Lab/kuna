@file:JvmName(name = "TestTopLevelModality")

package pkg

public val GREETING: String = "hello"

public var counter: Int = 3
   public get() {
      return field + 1
   }

   private set

public var limit: Long = 8L
   public set(value) {
      field = if (value < 0L) 0L else value
   }


public fun bumpCounter(): Int {
   counter = counter + 1
   return counter
}
