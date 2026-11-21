package pkg

public class TestMethodObjectReturnType {
   @JvmOverloads
   public fun <T> pick(first: T, second: T, takeSecond: Boolean = false): T {
      return (T)(if (takeSecond) second else first)
   }

   @JvmOverloads
   public fun describe(value: Any?, prefix: String = "v"): Any? {
      return if (value == null) null else "$prefix$value"
   }

   @JvmOverloads
   fun <T> pick(first: T, second: T): T {
      pick$default(this, first, second, false, 4, null)
   }

   @JvmOverloads
   fun describe(value: Any?) {
      describe$default(this, value, null, 2, null)
   }
}
