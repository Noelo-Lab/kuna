package pkg

import pkg.lib.ExtBox
import pkg.lib.describe
import pkg.lib.firstChar
import pkg.lib.half
import pkg.lib.label
import pkg.lib.pow2
import pkg.lib.shout
import pkg.lib.tag
import pkg.lib.wrap
import pkg.lib.ExtBox.Companion.toBox
import pkg.lib.ExtTools.twiceOf

public class TestExtensionCall(s: String) {
   public final val s: String

   init {
      this.s = s
   }

   public fun functions(): Int {
      return this.s.shout().length + this.s.wrap(2).length + (this.s.length pow2 3)
   }

   public fun properties(): Int {
      this.s.tag = 4
      return this.s.firstChar.code + this.s.tag + this.s.length.half
   }

   public fun members(): Int {
      return this.s.toBox().v + this.s.twiceOf().length
   }

   public fun narrower(): String {
      val var10000: String = this.s.describe()
      val var10001: String = this.s
      return "$var10000${(var10001 as CharSequence).describe()}"
   }

   public fun memberWins(b: ExtBox): String {
      return "${(b as Any).label()}${b.label()}${b.scaledBy(3)}"
   }
}
