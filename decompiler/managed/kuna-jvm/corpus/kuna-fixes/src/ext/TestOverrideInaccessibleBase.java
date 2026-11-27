package ext;

public abstract class TestOverrideInaccessibleBase {
   Object parser() {
      return null;
   }

   private int secret() {
      return 0;
   }

   protected int size() {
      return 1;
   }

   public String name() {
      return "base";
   }
}
