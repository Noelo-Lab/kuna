package pkg

public class TestKtFunctionParameterName(id: Int) {
   public final val id: Int

   init {
      this.id = var1
   }

   public fun plus(other: TestKtFunctionParameterName, extra: Int): Int {
      return this.id + var1.id + var2
   }

   public open operator fun equals(other: Any?): Boolean {
      return var1 is TestKtFunctionParameterName && (var1 as TestKtFunctionParameterName).id == this.id
   }

   public open fun hashCode(): Int {
      return this.id
   }

   public fun String.tagged(prefix: String): String {
      return "$var2$var1${this.id}"
   }

   public fun shift(by: Int): Int {
      return var1 + this.id
   }
}
