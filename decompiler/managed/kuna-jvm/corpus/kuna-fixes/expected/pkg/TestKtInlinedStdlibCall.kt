package pkg

import java.io.Closeable
import java.util.ArrayList
import java.util.Locale
import kotlin.jvm.internal.SourceDebugExtension

@SourceDebugExtension(["SMAP\nTestKtInlinedStdlibCall.kt\nKotlin\n*S Kotlin\n*F\n+ 1 TestKtInlinedStdlibCall.kt\npkg/TestKtInlinedStdlibCall\n+ 2 fake.kt\nkotlin/jvm/internal/FakeKt\n+ 3 _Collections.kt\nkotlin/collections/CollectionsKt___CollectionsKt\n*L\n1#1,55:1\n1#2:56\n1557#3:57\n1628#3,3:58\n1557#3:61\n1628#3,3:62\n1557#3:65\n1628#3,3:66\n*S KotlinDebug\n*F\n+ 1 TestKtInlinedStdlibCall.kt\npkg/TestKtInlinedStdlibCall\n*L\n46#1:57\n46#1:58,3\n48#1:61\n48#1:62,3\n50#1:65\n50#1:66,3\n*E\n"])
public class TestKtInlinedStdlibCall {
   public fun format(name: String, n: Int): String {
      return String.format("%s-%d", name, n)
   }

   public fun formatLocale(n: Int): String {
      return String.format(Locale.US, "%08x", n)
   }

   public fun formatReceiver(fmt: String, n: Int): String {
      return fmt.format(n)
   }

   public fun radix(i: Int, l: Long, b: Byte): String {
      return "${i.toString(16)}${l.toString(2)}${b.toString(16)}"
   }

   public fun parse(s: String): Int {
      return s.toInt(16) + s.toLong(8).toInt()
   }

   public fun range(a: ByteArray, from: Int): ByteArray {
      return a.copyOfRange(from, a.size)
   }

   public fun read(r: pkg.TestKtInlinedStdlibCall.Res): String {
      return r.use {
         it.text()
      }
   }

   public fun readNamed(r: pkg.TestKtInlinedStdlibCall.Res): Int {
      var var12: Int = r.use { res ->
         res.text().length + 1
      }

      return var12 * 2
   }

   public fun readInto(r: pkg.TestKtInlinedStdlibCall.Res, out: MutableList<String>) {
      r.use { res ->
         out.add(res.text())
         out.add(res.name)
      }
   }

   public fun firstLine(r: pkg.TestKtInlinedStdlibCall.Res): String {
      r.use {
         return it.text()
      }
   }

   public fun names(xs: List<pkg.TestKtInlinedStdlibCall.Res>): List<String> {
      return xs.map { it.name }
   }

   public fun lengths(xs: List<String>): List<Int> {
      return xs.map { s -> s.length + 1 }
   }

   public fun trimmed(xs: List<String>): List<String> {
      val `$this$map$iv`: Iterable<String> = xs as Iterable<String>
      val `destination$iv$iv`: MutableCollection<String> = ArrayList(CollectionsKt.collectionSizeOrDefault(`$this$map$iv`, 10))

      for (`item$iv$iv` in `$this$map$iv`) {
         val var10000: String = StringsKt.trim((`item$iv$iv` as String) as CharSequence).toString().toLowerCase(Locale.ROOT)
         `destination$iv$iv`.add(var10000)
      }

      return `destination$iv$iv` as List<String>
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
