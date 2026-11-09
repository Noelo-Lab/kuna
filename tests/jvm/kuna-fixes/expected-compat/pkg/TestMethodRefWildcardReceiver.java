package pkg;

public class TestMethodRefWildcardReceiver {
   private TestMethodRefWildcardReceiver.Pipe<Class<?>> types;

   TestMethodRefWildcardReceiver.Pipe<Object> objects(TestMethodRefWildcardReceiver.Factory<?> factory) {
      // $VF: Couldn't be decompiled
      // Bytecode:
      // 00: aload 0
      // 01: getfield pkg/TestMethodRefWildcardReceiver.types Lpkg/TestMethodRefWildcardReceiver$Pipe;
      // 04: aload 1
      // 05: dup
      // 06: invokestatic java/util/Objects.requireNonNull (Ljava/lang/Object;)Ljava/lang/Object;
      // 09: pop
      // 0a: invokedynamic apply (Lpkg/TestMethodRefWildcardReceiver$Factory;)Lpkg/TestMethodRefWildcardReceiver$Fn; bsm=java/lang/invoke/LambdaMetafactory.metafactory (Ljava/lang/invoke/MethodHandles$Lookup;Ljava/lang/String;Ljava/lang/invoke/MethodType;Ljava/lang/invoke/MethodType;Ljava/lang/invoke/MethodHandle;Ljava/lang/invoke/MethodType;)Ljava/lang/invoke/CallSite; args=[ (Ljava/lang/Object;)Ljava/lang/Object;, pkg/TestMethodRefWildcardReceiver$Factory.create (Ljava/lang/Class;)Ljava/lang/Object;, (Ljava/lang/Class;)Ljava/lang/Object; ]
      // 0f: invokeinterface pkg/TestMethodRefWildcardReceiver$Pipe.map (Lpkg/TestMethodRefWildcardReceiver$Fn;)Lpkg/TestMethodRefWildcardReceiver$Pipe; 2
      // 14: areturn
   }

   TestMethodRefWildcardReceiver.Pipe<TestMethodRefWildcardReceiver.Shape<?>> shapes(TestMethodRefWildcardReceiver.ShapeFactory<?> factory) {
      // $VF: Couldn't be decompiled
      // Bytecode:
      // 00: aload 0
      // 01: getfield pkg/TestMethodRefWildcardReceiver.types Lpkg/TestMethodRefWildcardReceiver$Pipe;
      // 04: aload 1
      // 05: dup
      // 06: invokestatic java/util/Objects.requireNonNull (Ljava/lang/Object;)Ljava/lang/Object;
      // 09: pop
      // 0a: invokedynamic apply (Lpkg/TestMethodRefWildcardReceiver$ShapeFactory;)Lpkg/TestMethodRefWildcardReceiver$Fn; bsm=java/lang/invoke/LambdaMetafactory.metafactory (Ljava/lang/invoke/MethodHandles$Lookup;Ljava/lang/String;Ljava/lang/invoke/MethodType;Ljava/lang/invoke/MethodType;Ljava/lang/invoke/MethodHandle;Ljava/lang/invoke/MethodType;)Ljava/lang/invoke/CallSite; args=[ (Ljava/lang/Object;)Ljava/lang/Object;, pkg/TestMethodRefWildcardReceiver$ShapeFactory.create (Ljava/lang/Class;)Lpkg/TestMethodRefWildcardReceiver$Shape;, (Ljava/lang/Class;)Lpkg/TestMethodRefWildcardReceiver$Shape; ]
      // 0f: invokeinterface pkg/TestMethodRefWildcardReceiver$Pipe.map (Lpkg/TestMethodRefWildcardReceiver$Fn;)Lpkg/TestMethodRefWildcardReceiver$Pipe; 2
      // 14: areturn
   }

   TestMethodRefWildcardReceiver.Pipe<Object> objectsOf(TestMethodRefWildcardReceiver.Factory<String> factory) {
      return this.types.map(factory::create);
   }

   static class Factory<T> {
      T create(Class<?> type) {
         return null;
      }
   }

   interface Fn<T, R> {
      R apply(T var1);
   }

   interface Pipe<T> {
      <R> TestMethodRefWildcardReceiver.Pipe<R> map(TestMethodRefWildcardReceiver.Fn<? super T, ? extends R> var1);
   }

   interface Shape<T> {
   }

   static class ShapeFactory<S extends TestMethodRefWildcardReceiver.Shape<?>> {
      S create(Class<?> type) {
         return null;
      }
   }
}
