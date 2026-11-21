package pkg;

import pkglib.Filters;

public class TestInheritedMemberType extends TestInheritedMemberBase implements Filters {
   public static class KeyedBucket extends TestInheritedMemberBase.BaseBucket implements Filters.Bucket {
      public String key() {
         return "k";
      }
   }

   public Filters.Bucket first() {
      return new KeyedBucket();
   }

   static String read(pkgio.Reader reader) {
      return reader.line();
   }
}
