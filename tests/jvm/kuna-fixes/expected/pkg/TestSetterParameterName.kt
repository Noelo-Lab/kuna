package pkg

public class TestSetterParameterName {
   public final var total: Long = 0L
      public set(value) {
         field = value
      }


   public final var label: String = ""
      public set(value) {
         field = value
      }


   public final var count: Int = 0
      public set(amount) {
         field = if (amount < 0) 0 else amount
      }

}
