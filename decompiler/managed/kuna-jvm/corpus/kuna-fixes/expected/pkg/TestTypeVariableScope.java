package pkg;

public class TestTypeVariableScope<T> {
   T mine;
   T[] array;
   TestTypeVariableScope.Box raw;

   int own(TestTypeVariableScope<T> var1) {
      T var2 = this.mine;
      T var3 = var1.mine;
      int var4 = var2.hashCode() + var3.hashCode();

      for (T var8 : this.array) {
         var4 += var8.hashCode();
      }

      return var4;
   }

   int rawReceiver() {
      Object var1 = this.raw.value;
      int var2 = var1.hashCode();

      for (Object var6 : this.raw.all) {
         var2 += var6.hashCode();
      }

      return var2;
   }

   int otherArguments(TestTypeVariableScope.Box<String> var1, TestTypeVariableScope<String> var2) {
      Object var3 = var1.value;
      Object var4 = var2.mine;
      return var3.hashCode() + var4.hashCode();
   }

   <T> int hidden(T var1) {
      Object var2 = this.mine;
      Object var3 = var1;
      return var2.hashCode() + var3.hashCode();
   }

   <T> TestTypeVariableScope.Run hiddenInLambda(T var1) {
      return () -> {
         Object var2 = this.mine;
         return var2.hashCode() + var1.hashCode();
      };
   }

   int innerField(TestTypeVariableScope<String>.Inner var1) {
      Object var2 = var1.inner;
      return var2.hashCode();
   }

   static class Box<T> {
      T value;
      T[] all;
   }

   class Inner {
      T inner;
   }

   interface Run {
      int run();
   }
}
