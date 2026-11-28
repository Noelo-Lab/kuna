package pkg;

public class TestLambdaTypeVariableTarget {
  static final Chunked HEADER = depth -> concat(single((Chunk) (o, d) -> o.put("#")), single(new Leaf()));

  interface Out {
    Out put(String text);
  }

  interface Chunk {
    Out write(Out out, int depth);
  }

  interface Chunked {
    It<? extends Chunk> chunks(int depth);
  }

  interface It<T> {
    T next();
  }

  static class Leaf implements Chunk {
    public Out write(Out out, int depth) {
      return out;
    }
  }

  static <T> It<T> single(T element) {
    return () -> element;
  }

  @SafeVarargs
  static <T> It<T> concat(It<? extends T>... parts) {
    return parts[0]::next;
  }

  It<? extends Chunk> leaves(It<Leaf> leaves) {
    return concat(single((Chunk) (o, d) -> o.put("[")), leaves, single((Chunk) (o, d) -> o.put("]")));
  }

  It<? extends Chunk> wrapped(Chunked inner, int depth) {
    return concat(single((Chunk) (o, d) -> o.put("{")), inner.chunks(depth));
  }

  It<Chunk> typed(It<Leaf> leaves) {
    return concat(single((o, d) -> o.put("<")), leaves);
  }

  It<? extends Chunk> lambdasOnly() {
    return concat(single((o, d) -> o.put("(")), single((o, d) -> o.put(")")));
  }
}
