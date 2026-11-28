package pkg;

import java.util.Iterator;
import java.util.function.Function;

public class TestVarargsGenericRival {
   static TestVarargsGenericRival.Geno<TestVarargsGenericRival.IntGene> one(TestVarargsGenericRival.IntChromosome c) {
      return TestVarargsGenericRival.Geno.of(c);
   }

   static Function<TestVarargsGenericRival.Geno<TestVarargsGenericRival.IntGene>, Integer> decoder(TestVarargsGenericRival.IntChromosome c) {
      TestVarargsGenericRival.Geno<TestVarargsGenericRival.IntGene> g = TestVarargsGenericRival.Geno.of(c, c);
      return gt -> ((TestVarargsGenericRival.IntGene)gt.chromosome().gene()).value();
   }

   static TestVarargsGenericRival.Geno<TestVarargsGenericRival.IntGene> list(Iterable<TestVarargsGenericRival.IntChromosome> cs) {
      return TestVarargsGenericRival.Geno.of(cs);
   }

   interface Chromosome<G extends TestVarargsGenericRival.Gene<G>> extends Iterable<G> {
      G gene();
   }

   interface Gene<G extends TestVarargsGenericRival.Gene<G>> {
      int value();
   }

   static final class Geno<G extends TestVarargsGenericRival.Gene<G>> {
      TestVarargsGenericRival.Chromosome<G> chromosome() {
         return null;
      }

      @SafeVarargs
      static <G extends TestVarargsGenericRival.Gene<G>> TestVarargsGenericRival.Geno<G> of(
         TestVarargsGenericRival.Chromosome<G> first, TestVarargsGenericRival.Chromosome<G>... rest
      ) {
         return new TestVarargsGenericRival.Geno<>();
      }

      static <G extends TestVarargsGenericRival.Gene<G>> TestVarargsGenericRival.Geno<G> of(
         Iterable<? extends TestVarargsGenericRival.Chromosome<G>> chromosomes
      ) {
         return new TestVarargsGenericRival.Geno<>();
      }
   }

   static final class IntChromosome implements TestVarargsGenericRival.Chromosome<TestVarargsGenericRival.IntGene> {
      public TestVarargsGenericRival.IntGene gene() {
         return new TestVarargsGenericRival.IntGene();
      }

      public Iterator<TestVarargsGenericRival.IntGene> iterator() {
         return null;
      }
   }

   static final class IntGene implements TestVarargsGenericRival.Gene<TestVarargsGenericRival.IntGene> {
      @Override
      public int value() {
         return 1;
      }
   }
}
