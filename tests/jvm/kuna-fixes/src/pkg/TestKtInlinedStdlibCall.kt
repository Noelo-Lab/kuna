package pkg

import java.io.Closeable
import java.util.Locale

class TestKtInlinedStdlibCall {
   class Res(val name: String) : Closeable {
      fun text(): String = name

      override fun close() {}
   }

   fun format(name: String, n: Int): String = String.format("%s-%d", name, n)

   fun formatLocale(n: Int): String = String.format(Locale.US, "%08x", n)

   fun formatReceiver(fmt: String, n: Int): String = fmt.format(n)

   fun radix(i: Int, l: Long, b: Byte): String = i.toString(16) + l.toString(2) + b.toString(16)

   fun parse(s: String): Int = s.toInt(16) + s.toLong(8).toInt()

   fun range(a: ByteArray, from: Int): ByteArray = a.copyOfRange(from, a.size)

   fun read(r: Res): String = r.use { it.text() }

   fun readNamed(r: Res): Int {
      val n = r.use { res ->
         val t = res.text()
         t.length + 1
      }
      return n * 2
   }

   fun readInto(r: Res, out: MutableList<String>) {
      r.use { res ->
         out.add(res.text())
         out.add(res.name)
      }
   }

   fun firstLine(r: Res): String {
      r.use { return it.text() }
   }

   fun names(xs: List<Res>): List<String> = xs.map { it.name }

   fun lengths(xs: List<String>): List<Int> = xs.map { s -> s.length + 1 }

   fun trimmed(xs: List<String>): List<String> = xs.map { s ->
      val t = s.trim()
      t.lowercase()
   }
}
