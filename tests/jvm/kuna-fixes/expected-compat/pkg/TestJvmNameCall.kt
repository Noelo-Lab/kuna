package pkg

import pkg.lib.Counter
import pkg.lib.Meters
import pkg.lib.SubCounter

public class TestJvmNameCall(c: Counter, s: SubCounter) {
   public final val c: Counter
   public final val s: SubCounter

   init {
      this.c = c
      this.s = s
   }

   public fun internalMember(): Int {
      return this.c.bump$main() + this.s.bump$main()
   }

   public fun renamed(): Int {
      return this.c.sizeOf() + this.c.getByte(1)
   }

   public fun inherited(): Int {
      this.s.setClosed(true)
      return if (this.s.getClosed()) this.s.sizeOf() else this.s.getCount()
   }

   public fun mangled(): Double {
      return this.c.take_dXQi75k/* $VF was: take-dXQi75k */(Meters.constructor_impl/* $VF was: constructor-impl */(2.0)) + this.c.take(3.0)
   }
}
