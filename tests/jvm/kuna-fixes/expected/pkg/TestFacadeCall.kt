package pkg

import pkg.lib.LIMIT
import pkg.lib.greet
import pkg.lib.lonely
import pkg.lib.names

public class TestFacadeCall(base: Int) {
   public final val base: Int

   init {
      this.base = base
   }

   public fun plain(): Int {
      return pkg.lib.twice(this.base) + lonely()
   }

   public fun property(): Int {
      pkg.lib.counter = pkg.lib.counter + this.base
      return pkg.lib.counter + LIMIT + names.size
   }

   public fun defaults(): String {
      return "${greet()}${greet("y")}"
   }

   public fun shadowedByMember(): Int {
      return pkg.lib.twice(this.base) + this.twice()
   }

   public fun twice(): Int {
      return this.base * 4
   }

   public fun shadowedByLocal(): Int {
      val counter: Int = this.twice()
      Integer.hashCode(counter)
      return counter * counter + pkg.lib.counter
   }
}
