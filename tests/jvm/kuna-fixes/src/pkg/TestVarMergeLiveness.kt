package pkg

class TestVarMergeLiveness {
   class Key(val v: Int)

   class Node(val size: Int)

   fun find(k: Key): Node? = if (k.v > 0) Node(k.v) else null

   fun empty(): Node = Node(0)

   fun larger(a: Key?, b: Key?): Node {
      val first = a?.let { find(it) } ?: empty()
      val second = b?.let { find(it) } ?: empty()
      return if (first.size > second.size) first else second
   }
}
