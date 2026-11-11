package pkg

public class TestSetterParameterName {
   public final var total: Long
      public final set(value) {
         this.total = var1
      }


   public final var label: String = ""
      public final set(value) {
         this.label = var1
      }


   public final var count: Int
      public final set(amount) {
         this.count = if (amount < 0) 0 else amount
      }

}
