package pkg

import java.util.Arrays
import kotlin.jvm.internal.SourceDebugExtension

@SourceDebugExtension(["SMAP\nTestKtLegacyToTypedArray.kt\nKotlin\n*S Kotlin\n*F\n+ 1 TestKtLegacyToTypedArray.kt\npkg/TestKtLegacyToTypedArray\n+ 2 ArraysJVM.kt\nkotlin/collections/ArraysKt__ArraysJVMKt\n*L\n1#1,45:1\n37#2,2:46\n37#2,2:48\n37#2,2:50\n37#2,2:52\n37#2,2:54\n*S KotlinDebug\n*F\n+ 1 TestKtLegacyToTypedArray.kt\npkg/TestKtLegacyToTypedArray\n*L\n7#1:46,2\n11#1:48,2\n23#1:50,2\n27#1:52,2\n41#1:54,2\n*E\n"])
public class TestKtLegacyToTypedArray {
   public final var items: Array<String> = arrayOfNulls(0)

   public fun names(list: List<String>): Array<String> {
      return (list as Collection<String>).toTypedArray()
   }

   public fun joined(list: List<String>): String {
      val var2: Array<String> = (list as Collection<String>).toTypedArray()
      return this.join(Arrays.copyOf(var2, var2.size) as Array<String>)
   }

   public fun join(vararg parts: String): String {
      var res: String = ""

      for (p in parts) {
         res = "$res$p"
      }

      return res
   }

   public fun keep(list: Collection<String>) {
      this.items = list.toTypedArray()
   }

   public fun objects(list: List<String>): Array<Any> {
      return (list as Collection<String>).toTypedArray<Any>()
   }

   public fun count(set: Set<Int>): Int {
      val array: Array<Int> = (set as Collection<Int>).toTypedArray()
      return array.size + array[0]
   }

   @SourceDebugExtension(["SMAP\nTestKtLegacyToTypedArray.kt\nKotlin\n*S Kotlin\n*F\n+ 1 TestKtLegacyToTypedArray.kt\npkg/TestKtLegacyToTypedArray\$Modern\n+ 2 ArraysJVM.kt\nkotlin/collections/ArraysKt__ArraysJVMKt\n*L\n1#1,45:1\n37#2,2:46\n37#2,2:48\n*S KotlinDebug\n*F\n+ 1 TestKtLegacyToTypedArray.kt\npkg/TestKtLegacyToTypedArray\$Modern\n*L\n32#1:46,2\n36#1:48,2\n*E\n"])
   public class Modern {
      public fun names(list: List<String>): Array<String> {
         return (list as Collection<String>).toTypedArray()
      }

      public fun objects(list: List<String>): Int {
         return (list as Collection<String>).toTypedArray<Any>().size
      }
   }
}
