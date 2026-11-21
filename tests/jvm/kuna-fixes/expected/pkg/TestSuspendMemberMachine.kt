package pkg

public class TestSuspendMemberMachine(src: MemberSource) {
   private final val src: MemberSource
   public final var total: Long = 0L

   init {
      this.src = src
   }

   public suspend fun addAll(n: Int): Long {
      repeat(n) { i ->
         this.total = this.total + (this.src.next() as Number).toLong()
      }

      return this.total
   }

   public suspend fun label(prefix: String): String {
      val a: Int = this.src.next()
      return "$prefix${this.src.text(a)}${this.total}"
   }
}
