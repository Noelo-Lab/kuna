package pkg

public class TestSyntheticAccessorCall(base: Int) {
   public final val base: Int

   init {
      this.base = base
   }

   public fun read(): String {
      return "${label(this.base.doubled())}${PREFIX}"
   }

   public fun write(): Int {
      hits = hits + this.base
      return hits
   }
}
