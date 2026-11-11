package pkg

import pkg.lib.ExtBox
import pkg.lib.ExtBox.Companion.toBox
import pkg.lib.ExtTools.twiceOf
import pkg.lib.firstChar
import pkg.lib.half
import pkg.lib.pow2
import pkg.lib.shout
import pkg.lib.tag
import pkg.lib.wrap
import pkg.lib.describe
import pkg.lib.label

class TestExtensionCall(val s: String) {
    fun functions(): Int = s.shout().length + s.wrap(2).length + (s.length pow2 3)

    fun properties(): Int {
        s.tag = 4
        return s.firstChar.code + s.tag + s.length.half
    }

    fun members(): Int = s.toBox().v + s.twiceOf().length

    fun narrower(): String = s.describe() + (s as CharSequence).describe()

    fun memberWins(b: ExtBox): String = (b as Any).label() + b.label() + b.scaledBy(3)
}
