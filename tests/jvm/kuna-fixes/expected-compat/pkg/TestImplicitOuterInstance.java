package pkg;

import java.util.function.Supplier;

public class TestImplicitOuterInstance<T> {
   T value;

   TestImplicitOuterInstance<T>.Entry direct() {
      return new TestImplicitOuterInstance.Entry();
   }

   static <T> TestImplicitOuterInstance<T>.Entry of(TestImplicitOuterInstance<T> t) {
      return t.new Entry();
   }

   class Entry {
      T get() {
         return TestImplicitOuterInstance.this.value;
      }
   }

   class Sub extends TestImplicitOuterInstance<T> {
      TestImplicitOuterInstance<T>.Entry outer() {
         return TestImplicitOuterInstance.this.new Entry();
      }

      TestImplicitOuterInstance<T>.Entry own() {
         return new TestImplicitOuterInstance.Entry();
      }
   }

   class View {
      TestImplicitOuterInstance<T>.Entry make() {
         return TestImplicitOuterInstance.this.new Entry();
      }

      TestImplicitOuterInstance<T>.Entry qualified() {
         return TestImplicitOuterInstance.this.new Entry();
      }

      Supplier<TestImplicitOuterInstance<T>.Entry> later() {
         return () -> TestImplicitOuterInstance.this.new Entry();
      }

      Object anonymous() {
         return new Object() {
            TestImplicitOuterInstance<T>.Entry entry() {
               return TestImplicitOuterInstance.this.new Entry();
            }
         };
      }

      class Deeper {
         TestImplicitOuterInstance<T>.Entry make() {
            return TestImplicitOuterInstance.this.new Entry();
         }

         TestImplicitOuterInstance<T>.View view() {
            return TestImplicitOuterInstance.this.new View();
         }
      }
   }
}
