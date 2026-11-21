package pkg

public class TestPropertyInitializerKey {
   public final val items: Array<Int>

   public fun name(i: Int): String {
      return "${this.items[i]}${EMPTY.length}"
   }

   public companion object {
      private final val EMPTY: Array<Any?>
   }
}
