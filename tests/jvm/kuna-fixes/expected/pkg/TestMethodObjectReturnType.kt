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
}
