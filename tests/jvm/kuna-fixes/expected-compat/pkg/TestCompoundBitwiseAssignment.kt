package pkg

public class TestCompoundBitwiseAssignment {
   public final var flags: Int
      internal set

   public final var mask: Long
      internal set

   public final var ok: Boolean
      internal set

   public final val words: IntArray = IntArray(4)
   public final val bytes: ByteArray = ByteArray(4)

   public fun mix(a: Int, b: Int): Long {
      this.flags |= 1 shl b
      this.mask |= 1L shl a
      this.flags ^= a
      this.mask >>>= 3
      this.flags <<= a and 7
      this.mask = this.mask and this.mask - 1L
      this.ok |= a > b
      this.ok = this.ok and this.flags != 0
      this.words[1] = this.words[1] or a
      this.words[a and 3] = this.words[a and 3] xor b
      this.bytes[2] = (byte)(this.bytes[2] or a)
      return this.flags + this.mask + this.words[1] + this.bytes[2] + (if (this.ok) 1 else 0)
   }
}
