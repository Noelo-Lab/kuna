package pkg

public class TestKtFunctionParameterName(id: Int) {
   public final val id: Int

   init {
      this.id = id
   }

   public fun plus(other: TestKtFunctionParameterName, extra: Int): Int {
      return this.id + other.id + extra
   }

   public open operator fun equals(other: Any?): Boolean {
      return other is TestKtFunctionParameterName && (other as TestKtFunctionParameterName).id == this.id
   }

   public open fun hashCode(): Int {
      return this.id
   }

   public fun String.tagged(prefix: String): String {
      return "$prefix$this${this@TestKtFunctionParameterName.id}"
   }

   public fun shift(by: Int): Int {
      return by + this.id
   }
}
