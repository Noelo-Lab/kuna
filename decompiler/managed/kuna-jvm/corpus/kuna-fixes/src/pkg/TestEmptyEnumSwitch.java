package pkg;

public class TestEmptyEnumSwitch {
   public void empty(EmptySwitchColor c) {
      switch (c) {
      }
   }

   public void twice(EmptySwitchColor c, EmptySwitchColor d) {
      switch (c) {
      }
      switch (d) {
      }
   }
}

enum EmptySwitchColor {
   RED,
   GREEN;
}
