package pkg

import kotlin.jvm.internal.SourceDebugExtension

@SourceDebugExtension(["SMAP\nTestMembersWithoutMetadata.kt\nKotlin\n*S Kotlin\n*F\n+ 1 TestMembersWithoutMetadata.kt\npkg/TestMembersWithoutMetadataKt$cell$1\n+ 2 TestMembersWithoutMetadata.kt\npkg/TestMembersWithoutMetadataKt\n*L\n1#1,16:1\n24#2:17\n*E\n"])
/* $VF: local visibility outside of method */ internal class `TestMembersWithoutMetadataKt$named$$inlined$cell$1` : NoMetaCell {
   public constructor(`$name`: java.lang.String, var2: Int) : super(`$name`, 4) {
      this.`$n$inlined` = var2
   }

   public override fun get(): Int {
      return this.`$n$inlined` * 2 + this.getWidth()
   }
}
