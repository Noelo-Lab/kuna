package pkg

public class NoMetaBag(src: NoMetaReader, size: Int) {
   public final val src: NoMetaReader
   public final val size: Int

   init {
      this.src = src
      this.size = size
   }

   public fun view(): NoMetaReader {
      return object : NoMetaReader {
         public final val first: Int

         {
            this.first = `$receiver`.src.read() + `$receiver`.size
         }

         public override fun read(): Int {
            return this.first
         }
      }
   }
}
