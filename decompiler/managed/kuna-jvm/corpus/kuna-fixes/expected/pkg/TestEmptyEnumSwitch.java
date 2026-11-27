package pkg;

public class TestEmptyEnumSwitch {
   public void empty(EmptySwitchColor c) {
      c.ordinal();
   }

   public void twice(EmptySwitchColor c, EmptySwitchColor d) {
      c.ordinal();
      d.ordinal();
   }
}
