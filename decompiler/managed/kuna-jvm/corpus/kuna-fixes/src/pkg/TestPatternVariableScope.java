package pkg;

public class TestPatternVariableScope {
   public static class Stream {
      void set(String s) {
      }
   }

   public static class Special extends Stream {
   }

   public Stream reuse(Object reuse, String s) {
      Stream stream;
      if (reuse instanceof Stream) {
         stream = (Stream) reuse;
      } else {
         stream = new Stream();
      }
      stream.set(s);
      return stream;
   }

   public Stream reuseChecked(Object reuse) {
      Special special;
      if (reuse instanceof Special) {
         special = (Special) reuse;
         if (special.hashCode() == 0) {
            special = new Special();
         }
      } else {
         special = new Special();
      }
      return special;
   }

   public void scoped(Object o) {
      if (o instanceof Stream) {
         Stream s = (Stream) o;
         s.set("scoped");
      }
   }
}
