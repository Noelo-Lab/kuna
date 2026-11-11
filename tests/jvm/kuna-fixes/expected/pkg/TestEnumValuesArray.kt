package pkg

import kotlin.enums.EnumEntries

public enum class TestEnumValuesArray(code: Int, label: String?) {
   FIRST(7, "one"),
   SECOND(-3, null),
   THIRD(42, "three");

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
      return `$ENTRIES`
   }
}
