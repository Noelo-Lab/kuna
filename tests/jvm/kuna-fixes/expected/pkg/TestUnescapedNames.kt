package pkg

import java.io.InputStream
import java.util.ArrayList
import kotlin.jvm.functions.Function1
import kotlin.jvm.functions.Function2
import kotlin.jvm.internal.SourceDebugExtension

@SourceDebugExtension(["SMAP\nTestUnescapedNames.kt\nKotlin\n*S Kotlin\n*F\n+ 1 TestUnescapedNames.kt\npkg/TestUnescapedNames\n+ 2 _Collections.kt\nkotlin/collections/CollectionsKt___CollectionsKt\n+ 3 _Maps.kt\nkotlin/collections/MapsKt___MapsKt\n*L\n1#1,38:1\n1557#2:39\n1628#2,3:40\n126#3:43\n153#3,3:44\n*S KotlinDebug\n*F\n+ 1 TestUnescapedNames.kt\npkg/TestUnescapedNames\n*L\n17#1:39\n17#1:40,3\n19#1:43\n19#1:44,3\n*E\n"])
public class TestUnescapedNames {
   private final var count: Int = 0

   public final val plusOne: (Int) -> Int = { `in` ->
      `in` + 1
   } as Function1

   public final val second: (Int, Int) -> Int = { _, b ->
      b
   } as Function2

   public fun IntArray.view(): AbstractList<Int> {
      return object : AbstractList<Int>() {
         public open val size: Int
            public get() {
               return `$this$view`.size
            }


         public override operator fun get(index: Int): Int {
            return `$this$view`[index]
         }
      }
   }

   public fun shift(xs: List<Int>): List<Int> {
      val `$this$mapTo$iv$iv`: java.lang.Iterable = xs as java.lang.Iterable
      val `destination$iv$iv`: java.util.Collection = ArrayList(CollectionsKt.collectionSizeOrDefault(xs as java.lang.Iterable, 10)) as java.util.Collection

      for (`item$iv$iv` in `$this$mapTo$iv$iv`) {
         `destination$iv$iv`.add((`item$iv$iv` as Number).toInt() * 2)
      }

      return `destination$iv$iv` as MutableList<Int>
   }

   public fun firsts(pairs: Map<Int, String>): List<Int> {
      val `destination$iv$iv`: java.util.Collection = ArrayList(pairs.size) as java.util.Collection

      for (`item$iv$iv` in pairs.entries) {
         `destination$iv$iv`.add((`item$iv$iv`.key as Number).toInt())
      }

      return `destination$iv$iv` as MutableList<Int>
   }

   public fun pickFirst(f: (Int, Int) -> Int): Int {
      return (f(3, 4) as Number).toInt()
   }

   public fun first(): Int {
      return this.pickFirst({ a, _ ->
         a
      })
   }

   public fun stdin(): InputStream {
      val var10000: InputStream = System.`in`
      return var10000
   }

   internal fun bump(): Int {
      val var1: Int = this.count++
      return this.count
   }

   public class Other {
      public fun use(t: TestUnescapedNames): Int {
         return t.`bump$main`()
      }
   }
}
