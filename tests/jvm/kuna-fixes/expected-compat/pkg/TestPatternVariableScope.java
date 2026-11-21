package pkg;

public class TestPatternVariableScope {
   public TestPatternVariableScope.Stream reuse(Object reuse, String s) {
      if (!(reuse instanceof TestPatternVariableScope.Stream stream)) {
         stream = new TestPatternVariableScope.Stream();
      }

      stream.set(s);
      return stream;
   }

   public TestPatternVariableScope.Stream reuseChecked(Object reuse) {
      if (reuse instanceof TestPatternVariableScope.Special special) {
         if (special.hashCode() == 0) {
            special = new TestPatternVariableScope.Special();
         }
      } else {
         special = new TestPatternVariableScope.Special();
      }

      return special;
   }

   public void scoped(Object o) {
      if (o instanceof TestPatternVariableScope.Stream s) {
         s.set("scoped");
      }
   }

   public static class Special extends TestPatternVariableScope.Stream {
   }

   public static class Stream {
      void set(String s) {
      }
   }
}
