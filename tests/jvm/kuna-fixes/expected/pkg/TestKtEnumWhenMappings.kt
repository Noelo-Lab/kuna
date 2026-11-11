package pkg

import kotlin.enums.EnumEntries

public class TestKtEnumWhenMappings {
   public fun name(c: pkg.TestKtEnumWhenMappings.Color): String {
      var var10000: String
      when (c) {
         TestKtEnumWhenMappings.Color.RED -> var10000 = "red"
         TestKtEnumWhenMappings.Color.GREEN -> var10000 = "green"
         TestKtEnumWhenMappings.Color.BLUE -> var10000 = "blue"
         else -> throw NoWhenBranchMatchedException()
      }

      return var10000
   }

   public fun reordered(c: pkg.TestKtEnumWhenMappings.Color): Int {
      var var10000: Byte
      when (c) {
         TestKtEnumWhenMappings.Color.RED -> var10000 = 20
         TestKtEnumWhenMappings.Color.BLUE -> var10000 = 10
         else -> var10000 = 30
      }

      return var10000.toInt()
   }

   public fun grouped(c: pkg.TestKtEnumWhenMappings.Color, s: pkg.TestKtEnumWhenMappings.Size): Int {
      var var4: Int
      when (c) {
         TestKtEnumWhenMappings.Color.RED, TestKtEnumWhenMappings.Color.BLUE -> var4 = 1 + 2
         else -> var4 = 1 + 3
      }

      when (s) {
         TestKtEnumWhenMappings.Size.LARGE -> var4 *= 5
         TestKtEnumWhenMappings.Size.SMALL -> var4 *= 7
         else -> {}
      }

      return var4
   }

   public fun nullable(c: pkg.TestKtEnumWhenMappings.Color?): String {
      var var10000: String
      when (c) {
         null -> var10000 = "none"
         TestKtEnumWhenMappings.Color.GREEN -> var10000 = "go"
         else -> var10000 = "stop"
      }

      return var10000
   }

   public fun nullableExhaustive(s: pkg.TestKtEnumWhenMappings.Size?): Int {
      var var10000: Byte
      when (s) {
         null -> var10000 = 0
         TestKtEnumWhenMappings.Size.LARGE -> var10000 = 3
         TestKtEnumWhenMappings.Size.SMALL -> var10000 = 1
         TestKtEnumWhenMappings.Size.MEDIUM -> var10000 = 2
         else -> throw NoWhenBranchMatchedException()
      }

      return var10000.toInt()
   }

   public enum class Color {
      RED,
      GREEN,
      BLUE;

      @JvmStatic
      fun getEntries(): EnumEntries<TestKtEnumWhenMappings.Color> {
         return `$ENTRIES`
      }
   }

   public enum class Size {
      SMALL,
      MEDIUM,
      LARGE;

      @JvmStatic
      fun getEntries(): EnumEntries<TestKtEnumWhenMappings.Size> {
         return `$ENTRIES`
      }
   }
}
