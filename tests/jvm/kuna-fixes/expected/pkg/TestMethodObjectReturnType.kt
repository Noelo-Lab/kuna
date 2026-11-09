package pkg

public class TestMethodObjectReturnType {
   @JvmOverloads
   public fun <T> pick(first: T, second: T, takeSecond: Boolean = false): T {
      return (if (takeSecond) second else first) as T
   }

   @JvmOverloads
   public fun describe(value: Any?, prefix: String = "v"): Any? {
      return if (value == null) null else "$prefix$value"
   }

   @JvmOverloads
   fun <T> pick(first: T, second: T): T {
      return pick$default(this, first, second, false, 4, null) as T
   }

   @JvmOverloads
   fun describe(value: Any?): Any? {
      return describe$default(this, value, null, 2, null)
   }
}
