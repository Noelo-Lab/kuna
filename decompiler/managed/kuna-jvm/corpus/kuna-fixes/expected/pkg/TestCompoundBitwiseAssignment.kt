package pkg

public class TestCompoundBitwiseAssignment {
   public final var flags: Int = 0
   public final var mask: Long = 0L
   public final var ok: Boolean = false
   public final val words: IntArray = IntArray(4)
   public final val bytes: ByteArray = ByteArray(4)

   public fun mix(a: Int, b: Int): Long {
      this.flags = this.flags or (1 shl b)
      this.mask = this.mask or (1L shl a)
      this.flags = this.flags xor a
      this.mask = this.mask ushr 3
      this.flags = this.flags shl (a and 7)
      this.mask = this.mask and this.mask - 1L
      this.ok = this.ok or (a > b)
      this.ok = this.ok and (this.flags != 0)
      this.words[1] = this.words[1] or a
      this.words[a and 3] = this.words[a and 3] xor b
      this.bytes[2] = (this.bytes[2].toInt() or a).toByte()
      return this.flags + this.mask + this.words[1] + this.bytes[2] + (if (this.ok) 1 else 0)
   }
}
