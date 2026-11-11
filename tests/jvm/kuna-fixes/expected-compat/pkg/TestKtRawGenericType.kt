package pkg

import java.util.ArrayList
import kotlin.jvm.functions.Function1
import kotlin.jvm.internal.SourceDebugExtension
import kotlin.jvm.internal.Ref.ObjectRef

@SourceDebugExtension(["SMAP\nTestKtRawGenericType.kt\nKotlin\n*S Kotlin\n*F\n+ 1 TestKtRawGenericType.kt\npkg/TestKtRawGenericType\n+ 2 _Collections.kt\nkotlin/collections/CollectionsKt___CollectionsKt\n*L\n1#1,58:1\n2632#2,3:59\n*S KotlinDebug\n*F\n+ 1 TestKtRawGenericType.kt\npkg/TestKtRawGenericType\n*L\n36#1:59,3\n*E\n"])
public class TestKtRawGenericType(names: MutableList<String>) {
   private final val names: MutableList<String>

   init {
      this.names = names
   }

   public fun findMatching(prefix: String): List<String> {
      val result: java.util.List = ArrayList() as java.util.List

      for (name in this.names) {
         if (name.length() > prefix.length()) {
            result.add(name)
         }
      }

      return result
   }

   public fun totalLength(prefix: String): Int {
      val matches: java.util.List = this.findMatching(prefix)
      var total: Int = 0
      val i: java.util.Iterator = matches.iterator()

      while (i.hasNext()) {
         total += (i.next() as java.lang.String).length()
      }

      return total + matches.size()
   }

   public fun prune(limit: Int) {
      val i: java.util.Iterator = this.names.iterator()

      while (i.hasNext()) {
         if ((i.next() as java.lang.String).length() > limit) {
            i.remove()
         }
      }
   }

   public fun noneEmpty(values: Iterable<String>): Boolean {
      val var10000: Boolean
      if (values is java.util.Collection && (values as java.util.Collection).isEmpty()) {
         var10000 = true
      } else {
         for (`element$iv` in values) {
            if (((`element$iv` as java.lang.String) as java.lang.CharSequence).length() == 0) {
               return false
            }
         }

         var10000 = true
      }

      return var10000
   }

   public fun depth(value: Any): Int {
      var c: Class = value.getClass()
      var n: Int = 0

      while (c != null) {
         n++
         c = c.getSuperclass()
      }

      return n
   }

   public fun lastSeen(values: List<String>): String? {
      val last: ObjectRef = ObjectRef()
      val visit: Function1 = { it: java.lang.String ->
         `$last`.element = it
         Unit.INSTANCE
      }

      for (v in values) {
         visit(v)
      }

      return last.element as java.lang.String
   }
}
