package pkg

import kotlin.jvm.internal.SourceDebugExtension

@SourceDebugExtension(["SMAP\nTestMembersWithoutMetadata.kt\nKotlin\n*S Kotlin\n*F\n+ 1 TestMembersWithoutMetadata.kt\npkg/TestMembersWithoutMetadataKt\$feed$1\n+ 2 TestMembersWithoutMetadata.kt\npkg/TestMembersWithoutMetadataKt\n*L\n1#1,20:1\n27#2,11:21\n*E\n"])
/* $VF: local visibility outside of method */ internal class `TestMembersWithoutMetadataKt$counting$$inlined$feed$1` : NoMetaFeed {
   public constructor(var1: Long, var3: NoMetaFeed, var4: Int) {
      this.`$step$inlined` = var1
      this.`$src$inlined` = var3
      this.`$start$inlined` = var4
   }

   public override suspend fun next(): Int {
      var total: Long = this.`$step$inlined`
      val a: Int = this.`$src$inlined`.next()
      total = total + a
      val var15: Double = 0.5 + a
      val var16: Float = 1.5F + a
      val seen: Boolean = a > this.`$start$inlined`
      val b: Int = this.`$src$inlined`.next()
      return (if (seen) b + total.toInt() + var15.toInt() + var16.toInt() else b) + 1
   }
}
