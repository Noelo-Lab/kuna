package pkg;

public class TestLambdaTypeVariableTarget {
   static final TestLambdaTypeVariableTarget.Chunked HEADER = depth -> concat(single((o, d) -> o.put("#")), single(new TestLambdaTypeVariableTarget.Leaf()));

   static <T> TestLambdaTypeVariableTarget.It<T> single(T element) {
      return () -> element;
   }

   @SafeVarargs
   static <T> TestLambdaTypeVariableTarget.It<T> concat(TestLambdaTypeVariableTarget.It<? extends T>... parts) {
      return parts[0]::next;
   }

   TestLambdaTypeVariableTarget.It<? extends TestLambdaTypeVariableTarget.Chunk> leaves(
      TestLambdaTypeVariableTarget.It<TestLambdaTypeVariableTarget.Leaf> leaves
   ) {
      return concat(single((o, d) -> o.put("[")), leaves, single((o, d) -> o.put("]")));
   }

   TestLambdaTypeVariableTarget.It<? extends TestLambdaTypeVariableTarget.Chunk> wrapped(TestLambdaTypeVariableTarget.Chunked inner, int depth) {
      return concat(single((o, d) -> o.put("{")), inner.chunks(depth));
   }

   TestLambdaTypeVariableTarget.It<TestLambdaTypeVariableTarget.Chunk> typed(TestLambdaTypeVariableTarget.It<TestLambdaTypeVariableTarget.Leaf> leaves) {
      return concat(single((o, d) -> o.put("<")), leaves);
   }

   TestLambdaTypeVariableTarget.It<? extends TestLambdaTypeVariableTarget.Chunk> lambdasOnly() {
      return concat(single((o, d) -> o.put("(")), single((o, d) -> o.put(")")));
   }

   interface Chunk {
      TestLambdaTypeVariableTarget.Out write(TestLambdaTypeVariableTarget.Out var1, int var2);
   }

   interface Chunked {
      TestLambdaTypeVariableTarget.It<? extends TestLambdaTypeVariableTarget.Chunk> chunks(int var1);
   }

   interface It<T> {
      T next();
   }

   static class Leaf implements TestLambdaTypeVariableTarget.Chunk {
      @Override
      public TestLambdaTypeVariableTarget.Out write(TestLambdaTypeVariableTarget.Out out, int depth) {
         return out;
      }
   }

   interface Out {
      TestLambdaTypeVariableTarget.Out put(String var1);
   }
}
