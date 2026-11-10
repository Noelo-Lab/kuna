package pkg

public class TestExplicitTypeArguments {
   public fun lastLabel(items: List<pkg.TestExplicitTypeArguments.Item>): String {
      return TestExplicitTypeArguments.Seqs.<TestExplicitTypeArguments.Item>lastOf(items).label()
   }

   public fun firstLabel(items: Array<pkg.TestExplicitTypeArguments.Item>): String {
      return TestExplicitTypeArguments.Seqs.firstOf(items).label()
   }

   public class Item(label: String) {
      private final val label: String

      init {
         this.label = label
      }

      public fun label(): String {
         return this.label
      }
   }

   public object Seqs {
      @JvmStatic
      public fun <T> lastOf(items: List<T>): T {
         return (T)items.get(items.size() - 1)
      }

      @JvmStatic
      public fun <T> firstOf(items: Array<T>): T {
         return (T)items[0]
      }
   }
}
