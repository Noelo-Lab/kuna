package pkg;

import pkglib.Filters;

public class TestInheritedMemberType extends TestInheritedMemberBase implements Filters {
   public pkglib.Filters.Bucket first() {
      return new TestInheritedMemberType.KeyedBucket();
   }

   static String read(pkgio.Reader reader) {
      return reader.line();
   }

   public static class KeyedBucket extends TestInheritedMemberBase.BaseBucket implements pkglib.Filters.Bucket {
      public String key() {
         return "k";
      }
   }
}
