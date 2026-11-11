package pkg

public class TestSecondaryConstructorLine(n: Int, s: String) {
   public final val n: Int
   public final val s: String

   init {
      this.n = n
      this.s = s
   }

   public constructor(n: Int) : this(n, "n")
   public open class Base(text: String) {
      public final val text: String

      init {
         this.text = text
      }

      public constructor(code: Int) : this(java.lang.String.valueOf(code))
      public fun size(): Int {
         return this.text.length()
      }

      public constructor() : this(0)   }
}
