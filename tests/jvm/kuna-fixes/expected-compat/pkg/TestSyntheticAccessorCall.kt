package pkg

public class TestSyntheticAccessorCall(base: Int) {
   public final val base: Int

   init {
      this.base = base
   }

   public fun read(): String {
      return "${TestSyntheticAccessorCallKt.access$label(TestSyntheticAccessorCallKt.access$doubled(this.base))}${TestSyntheticAccessorCallKt.access$getPREFIX$p()}"
   }

   public fun write(): Int {
      TestSyntheticAccessorCallKt.access$setHits$p(TestSyntheticAccessorCallKt.access$getHits$p() + this.base)
      return TestSyntheticAccessorCallKt.access$getHits$p()
   }
}
