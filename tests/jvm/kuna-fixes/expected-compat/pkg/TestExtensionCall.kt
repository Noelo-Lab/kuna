package pkg

import pkg.lib.ExtBox
import pkg.lib.ExtLib
import pkg.lib.ExtMulti
import pkg.lib.ExtTools

public class TestExtensionCall(s: String) {
   public final val s: String

   init {
      this.s = s
   }

   public fun functions(): Int {
      return ExtLib.shout(this.s).length() + ExtMulti.wrap(this.s, 2).length() + (this.s.length() pow2 3)
   }

   public fun properties(): Int {
      tag = this.s
      return firstChar + tag + ExtMulti.getHalf(this.s.length())
   }

   public fun members(): Int {
      return ExtBox.Companion.parse(this.s).v + ExtTools.INSTANCE.twiceOf(this.s).length()
   }

   public fun narrower(): String {
      val var10000: java.lang.String = ExtLib.describe(this.s)
      val var10001: java.lang.String = this.s
      return "$var10000${ExtLib.describe(var10001 as java.lang.CharSequence)}"
   }

   public fun memberWins(b: ExtBox): String {
      return "${ExtLib.label(b)}${b.label()}${b.scaledBy(3)}"
   }
}
