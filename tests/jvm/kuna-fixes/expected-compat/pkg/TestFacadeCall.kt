package pkg

import pkg.lib.FacadeLib
import pkg.lib.FacadeMulti

public class TestFacadeCall(base: Int) {
   public final val base: Int

   init {
      this.base = base
   }

   public fun plain(): Int {
      return FacadeLib.twice(this.base) + FacadeMulti.lonely()
   }

   public fun property(): Int {
      counter = counter + this.base
      return counter + FacadeLib.LIMIT + names.size()
   }

   public fun defaults(): String {
      return "${FacadeMulti.greet$default(null, 1, null)}${FacadeMulti.greet("y")}"
   }

   public fun shadowedByMember(): Int {
      return FacadeLib.twice(this.base) + this.twice()
   }

   public fun twice(): Int {
      return this.base * 4
   }

   public fun shadowedByLocal(): Int {
      val counter: Int = this.twice()
      Integer.hashCode(counter)
      return counter * counter + counter
   }
}
