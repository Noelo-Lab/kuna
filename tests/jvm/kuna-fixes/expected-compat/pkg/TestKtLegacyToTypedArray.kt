package pkg

import java.util.Arrays
import kotlin.jvm.internal.Intrinsics
import kotlin.jvm.internal.SourceDebugExtension

@SourceDebugExtension(["SMAP\nTestKtLegacyToTypedArray.kt\nKotlin\n*S Kotlin\n*F\n+ 1 TestKtLegacyToTypedArray.kt\npkg/TestKtLegacyToTypedArray\n+ 2 ArraysJVM.kt\nkotlin/collections/ArraysKt__ArraysJVMKt\n*L\n1#1,45:1\n37#2,2:46\n37#2,2:48\n37#2,2:50\n37#2,2:52\n37#2,2:54\n*S KotlinDebug\n*F\n+ 1 TestKtLegacyToTypedArray.kt\npkg/TestKtLegacyToTypedArray\n*L\n7#1:46,2\n11#1:48,2\n23#1:50,2\n27#1:52,2\n41#1:54,2\n*E\n"])
public class TestKtLegacyToTypedArray {
   public final var items: Array<String>
      internal set

   public fun names(list: List<String>): Array<String> {
      Intrinsics.checkParameterIsNotNull(list, "list")
      val var10000: Array<Any> = (list as java.util.Collection).toArray(arrayOfNulls(0))
      if (var10000 == null) {
         throw TypeCastException("null cannot be cast to non-null type kotlin.Array<T>")
      } else {
         return var10000 as Array<java.lang.String>
      }
   }

   public fun joined(list: List<String>): String {
      Intrinsics.checkParameterIsNotNull(list, "list")
      val var10001: Array<Any> = (list as java.util.Collection).toArray(arrayOfNulls(0))
      if (var10001 == null) {
         throw TypeCastException("null cannot be cast to non-null type kotlin.Array<T>")
      } else {
         return this.join(Arrays.copyOf(var10001 as Array<java.lang.String>, (var10001 as Array<java.lang.String>).length) as Array<java.lang.String>)
      }
   }

   public fun join(vararg parts: String): String {
      Intrinsics.checkParameterIsNotNull(parts, "parts")
      var res: java.lang.String = ""

      for (p in parts) {
         res = "$res$p"
      }

      return res
   }

   public fun keep(list: Collection<String>) {
      Intrinsics.checkParameterIsNotNull(list, "list")
      val var10001: Array<Any> = list.toArray(arrayOfNulls(0))
      if (var10001 == null) {
         throw TypeCastException("null cannot be cast to non-null type kotlin.Array<T>")
      } else {
         this.items = var10001 as Array<java.lang.String>
      }
   }

   public fun objects(list: List<String>): Array<Any> {
      Intrinsics.checkParameterIsNotNull(list, "list")
      return (list as java.util.Collection).toArray(arrayOfNulls(0))
   }

   public fun count(set: Set<Int>): Int {
      Intrinsics.checkParameterIsNotNull(set, "set")
      val var10000: Array<Any> = (set as java.util.Collection).toArray(arrayOfNulls(0))
      if (var10000 == null) {
         throw TypeCastException("null cannot be cast to non-null type kotlin.Array<T>")
      } else {
         return (var10000 as Array<Int>).length + (var10000 as Array<Int>)[0]
      }
   }

   @SourceDebugExtension(["SMAP\nTestKtLegacyToTypedArray.kt\nKotlin\n*S Kotlin\n*F\n+ 1 TestKtLegacyToTypedArray.kt\npkg/TestKtLegacyToTypedArray$Modern\n+ 2 ArraysJVM.kt\nkotlin/collections/ArraysKt__ArraysJVMKt\n*L\n1#1,45:1\n37#2,2:46\n37#2,2:48\n*S KotlinDebug\n*F\n+ 1 TestKtLegacyToTypedArray.kt\npkg/TestKtLegacyToTypedArray$Modern\n*L\n32#1:46,2\n36#1:48,2\n*E\n"])
   public class Modern {
      public fun names(list: List<String>): Array<String> {
         Intrinsics.checkParameterIsNotNull(list, "list")
         return (list as java.util.Collection).toArray(arrayOfNulls(0)) as Array<java.lang.String>
      }

      public fun objects(list: List<String>): Int {
         Intrinsics.checkParameterIsNotNull(list, "list")
         return (list as java.util.Collection).toArray(arrayOfNulls(0)).length
      }
   }
}
