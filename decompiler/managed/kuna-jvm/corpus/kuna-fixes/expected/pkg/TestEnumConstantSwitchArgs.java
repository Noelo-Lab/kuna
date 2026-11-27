package pkg;

public enum TestEnumConstantSwitchArgs {
   LOW(switch (TestEnumConstantSwitchArgs.Holder.level()) {
      case 0 -> 10;
      case 1 -> 20;
      default -> 30;
   }),
   HIGH(switch (TestEnumConstantSwitchArgs.Holder.level() + 1) {
      case 0 -> 1;
      default -> 2;
   });

   final int weight;

   TestEnumConstantSwitchArgs(int weight) {
      this.weight = weight;
   }

   static class Holder {
      static int cached = switch (level()) {
         case 0 -> 1;
         default -> 2;
      };
      static int after = 5;

      static int level() {
         return 1;
      }
   }
}
