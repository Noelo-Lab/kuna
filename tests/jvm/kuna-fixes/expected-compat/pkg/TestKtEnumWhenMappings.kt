package pkg

import kotlin.enums.EnumEntries

public class TestKtEnumWhenMappings {
   public fun name(c: pkg.TestKtEnumWhenMappings.Color): String {
      var var10000: java.lang.String
      when (TestKtEnumWhenMappings.WhenMappings.$EnumSwitchMapping$0[c.ordinal()]) {
         1 -> var10000 = "red"
         2 -> var10000 = "green"
         3 -> var10000 = "blue"
         else -> throw NoWhenBranchMatchedException()
      }

      return var10000
   }

   public fun reordered(c: pkg.TestKtEnumWhenMappings.Color): Int {
      var var10000: Byte
      when (TestKtEnumWhenMappings.WhenMappings.$EnumSwitchMapping$0[c.ordinal()]) {
         1 -> var10000 = 20
         2 -> var10000 = 30
         3 -> var10000 = 10
         else -> var10000 = 30
      }

      return var10000
   }

   public fun grouped(c: pkg.TestKtEnumWhenMappings.Color, s: pkg.TestKtEnumWhenMappings.Size): Int {
      var var4: Int
      when (TestKtEnumWhenMappings.WhenMappings.$EnumSwitchMapping$0[c.ordinal()]) {
         1, 3 -> var4 = 1 + 2
         2 -> var4 = 1 + 3
         else -> var4 = 1 + 3
      }

      when (TestKtEnumWhenMappings.WhenMappings.$EnumSwitchMapping$1[s.ordinal()]) {
         1 -> var4 *= 5
         2 -> var4 *= 7
         else -> {}
      }

      return var4
   }

   public fun nullable(c: pkg.TestKtEnumWhenMappings.Color?): String {
      var var10000: java.lang.String
      when (if (c == null) -1 else TestKtEnumWhenMappings.WhenMappings.$EnumSwitchMapping$0[c.ordinal()]) {
         -1 -> var10000 = "none"
         0, 1 -> var10000 = "stop"
         2 -> var10000 = "go"
         else -> var10000 = "stop"
      }

      return var10000
   }

   public fun nullableExhaustive(s: pkg.TestKtEnumWhenMappings.Size?): Int {
      var var10000: Byte
      when (if (s == null) -1 else TestKtEnumWhenMappings.WhenMappings.$EnumSwitchMapping$1[s.ordinal()]) {
         -1 -> var10000 = 0
         0 -> throw NoWhenBranchMatchedException()
         1 -> var10000 = 3
         2 -> var10000 = 1
         3 -> var10000 = 2
         else -> throw NoWhenBranchMatchedException()
      }

      return var10000
   }

   public enum class Color {
      RED,
      GREEN,
      BLUE;

      @JvmStatic
      fun getEntries(): EnumEntries<TestKtEnumWhenMappings.Color> {
         $ENTRIES
      }
   }

   public enum class Size {
      SMALL,
      MEDIUM,
      LARGE;

      @JvmStatic
      fun getEntries(): EnumEntries<TestKtEnumWhenMappings.Size> {
         $ENTRIES
      }
   }
}
