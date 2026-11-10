package pkg

public class TestOperatorLineBreak(firstComponentValue: String,
   secondComponentValue: String,
   thirdComponentValue: String,
   fourthComponentValue: String,
   fifthComponentValue: Int
) {
   private final val firstComponentValue: String
   private final val secondComponentValue: String
   private final val thirdComponentValue: String
   private final val fourthComponentValue: String
   private final val fifthComponentValue: Int

   init {
      this.firstComponentValue = firstComponentValue
      this.secondComponentValue = secondComponentValue
      this.thirdComponentValue = thirdComponentValue
      this.fourthComponentValue = fourthComponentValue
      this.fifthComponentValue = fifthComponentValue
   }

   public fun combined(): Int {
      return 31 *
            (
               31 * (31 * (31 * this.firstComponentValue.hashCode() + this.secondComponentValue.hashCode()) + this.thirdComponentValue.hashCode()) +
                  this.fourthComponentValue.hashCode()
            ) +
         this.fifthComponentValue
      }

   public fun total(extraAmountOfSomethingThatIsLong: Int): Int {
      return this.firstComponentValue.length() +
         this.secondComponentValue.length() +
         this.thirdComponentValue.length() +
         this.fourthComponentValue.length() +
         extraAmountOfSomethingThatIsLong
      }

   public fun remaining(base: Long): Long {
      return base * 1000L - this.firstComponentValue.length() * 86400L - this.secondComponentValue.length() * 3600L - this.thirdComponentValue.length()
   }

   public fun product(a: Int, b: Int, c: Int): Int {
      return (a + this.firstComponentValue.length() + this.secondComponentValue.length() + this.thirdComponentValue.length()) *
         (b + this.fourthComponentValue.length() + this.fifthComponentValue + c + a * b * c)
      }
}
