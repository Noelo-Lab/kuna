package pkg

import java.io.Closeable
import java.util.ArrayList
import java.util.Arrays
import java.util.Locale
import kotlin.jvm.internal.SourceDebugExtension

@SourceDebugExtension(["SMAP\nTestKtInlinedStdlibCall.kt\nKotlin\n*S Kotlin\n*F\n+ 1 TestKtInlinedStdlibCall.kt\npkg/TestKtInlinedStdlibCall\n+ 2 fake.kt\nkotlin/jvm/internal/FakeKt\n+ 3 _Collections.kt\nkotlin/collections/CollectionsKt___CollectionsKt\n*L\n1#1,55:1\n1#2:56\n1557#3:57\n1628#3,3:58\n1557#3:61\n1628#3,3:62\n1557#3:65\n1628#3,3:66\n*S KotlinDebug\n*F\n+ 1 TestKtInlinedStdlibCall.kt\npkg/TestKtInlinedStdlibCall\n*L\n46#1:57\n46#1:58,3\n48#1:61\n48#1:62,3\n50#1:65\n50#1:66,3\n*E\n"])
public class TestKtInlinedStdlibCall {
   public fun format(name: String, n: Int): String {
      val var5: Array<Any> = arrayOf(name, n)
      val var10000: java.lang.String = java.lang.String.format("%s-%d", Arrays.copyOf(var5, var5.length))
      return var10000
   }

   public fun formatLocale(n: Int): String {
      val var2: Locale = Locale.US
      val var5: Array<Any> = arrayOf(n)
      val var10000: java.lang.String = java.lang.String.format(var2, "%08x", Arrays.copyOf(var5, var5.length))
      return var10000
   }

   public fun formatReceiver(fmt: String, n: Int): String {
      val var5: Array<Any> = arrayOf(n)
      val var10000: java.lang.String = java.lang.String.format(fmt, Arrays.copyOf(var5, var5.length))
      return var10000
   }

   public fun radix(i: Int, l: Long, b: Byte): String {
      val var10000: java.lang.String = Integer.toString(i, CharsKt.checkRadix(16))
      val var10001: java.lang.String = java.lang.Long.toString(l, CharsKt.checkRadix(2))
      val var10002: java.lang.String = Integer.toString(b, CharsKt.checkRadix(16))
      return "$var10000$var10001$var10002"
   }

   public fun parse(s: String): Int {
      return Integer.parseInt(s, CharsKt.checkRadix(16)) + (int)java.lang.Long.parseLong(s, CharsKt.checkRadix(8))
   }

   public fun range(a: ByteArray, from: Int): ByteArray {
      return ArraysKt.copyOfRange(a, from, a.length)
   }

   public fun read(r: pkg.TestKtInlinedStdlibCall.Res): String {
      val var2: Closeable = r
      var var3: java.lang.Throwable = null

      try {
         return (var2 as TestKtInlinedStdlibCall.Res).text()
      } catch (var8: java.lang.Throwable) {
         var3 = var8
         throw var8
      } finally {
         CloseableKt.closeFinally(var2, var3)
      }
   }

   public fun readNamed(r: pkg.TestKtInlinedStdlibCall.Res): Int {
      val var3: Closeable = r
      var var4: java.lang.Throwable = null

      var var12: Int
      try {
         var12 = (var3 as TestKtInlinedStdlibCall.Res).text().length() + 1
      } catch (var10: java.lang.Throwable) {
         var4 = var10
         throw var10
      } finally {
         CloseableKt.closeFinally(var3, var4)
      }

      return var12 * 2
   }

   public fun readInto(r: pkg.TestKtInlinedStdlibCall.Res, out: MutableList<String>) {
      val var3: Closeable = r
      var var4: java.lang.Throwable = null

      try {
         val res: TestKtInlinedStdlibCall.Res = var3 as TestKtInlinedStdlibCall.Res
         out.add((var3 as TestKtInlinedStdlibCall.Res).text())
         val var11: Boolean = out.add(res.name)
      } catch (var9: java.lang.Throwable) {
         var4 = var9
         throw var9
      } finally {
         CloseableKt.closeFinally(var3, var4)
      }
   }

   public fun firstLine(r: pkg.TestKtInlinedStdlibCall.Res): String {
      val var2: Closeable = r
      var var3: java.lang.Throwable = null

      try {
         return (var2 as TestKtInlinedStdlibCall.Res).text()
      } catch (var9: java.lang.Throwable) {
         var3 = var9
         throw var9
      } finally {
         CloseableKt.closeFinally(var2, var3)
      }
   }

   public fun names(xs: List<pkg.TestKtInlinedStdlibCall.Res>): List<String> {
      val `$this$mapTo$iv$iv`: java.lang.Iterable = xs as java.lang.Iterable
      val `destination$iv$iv`: java.util.Collection = ArrayList(CollectionsKt.collectionSizeOrDefault(xs as java.lang.Iterable, 10)) as java.util.Collection

      for (`item$iv$iv` in `$this$mapTo$iv$iv`) {
         `destination$iv$iv`.add((`item$iv$iv` as TestKtInlinedStdlibCall.Res).name)
      }

      return `destination$iv$iv` as MutableList<java.lang.String>
   }

   public fun lengths(xs: List<String>): List<Int> {
      val `$this$mapTo$iv$iv`: java.lang.Iterable = xs as java.lang.Iterable
      val `destination$iv$iv`: java.util.Collection = ArrayList(CollectionsKt.collectionSizeOrDefault(xs as java.lang.Iterable, 10)) as java.util.Collection

      for (`item$iv$iv` in `$this$mapTo$iv$iv`) {
         `destination$iv$iv`.add((`item$iv$iv` as java.lang.String).length() + 1)
      }

      return `destination$iv$iv` as MutableList<Int>
   }

   public fun trimmed(xs: List<String>): List<String> {
      val `$this$mapTo$iv$iv`: java.lang.Iterable = xs as java.lang.Iterable
      val `destination$iv$iv`: java.util.Collection = ArrayList(CollectionsKt.collectionSizeOrDefault(xs as java.lang.Iterable, 10)) as java.util.Collection

      for (`item$iv$iv` in `$this$mapTo$iv$iv`) {
         val var10000: java.lang.String = StringsKt.trim((`item$iv$iv` as java.lang.String) as java.lang.CharSequence).toString().toLowerCase(Locale.ROOT)
         `destination$iv$iv`.add(var10000)
      }

      return `destination$iv$iv` as MutableList<java.lang.String>
   }

   public class Res(name: String) : Closeable {
      public final val name: String

      init {
         this.name = name
      }

      public fun text(): String {
         return this.name
      }

      public open fun close() {
      }
   }
}
