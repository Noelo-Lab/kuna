package pkg

import kotlin.enums.EnumEntries

public enum class TestEnumValuesArray(code: Int, label: String?) {
   FIRST,
   SECOND,
   THIRD;

   public final val code: Int
   public final val label: String?

   init {
      this.code = code
      this.label = label
   }

   public fun describe(): String {
      return "${this.label}=${this.code}"
   }

   @JvmStatic
   fun getEntries(): EnumEntries<TestEnumValuesArray> {
      $ENTRIES
   }
}
