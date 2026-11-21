package pkg;

import pkgio.Reader;
import pkglib.Filters;
import pkglib.Filters.Bucket;

public class TestInheritedMemberType extends TestInheritedMemberBase implements Filters {
   public Bucket first() {
      return new TestInheritedMemberType.KeyedBucket();
   }

   static String read(Reader reader) {
      return reader.line();
   }

   public static class KeyedBucket extends TestInheritedMemberBase.BaseBucket implements Bucket {
      public String key() {
         return "k";
      }
   }
}
