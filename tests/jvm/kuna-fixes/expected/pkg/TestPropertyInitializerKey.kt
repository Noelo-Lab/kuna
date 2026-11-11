package pkg

public class TestPropertyInitializerKey {
   public final val items: Array<Int> = arrayOf(1, 2, 3)

   public fun name(i: Int): String {
      return "${this.items[i]}${EMPTY.size}"
   }

   public companion object {
      private final val EMPTY: Array<Any?> = arrayOfNulls(0)
   }
}
