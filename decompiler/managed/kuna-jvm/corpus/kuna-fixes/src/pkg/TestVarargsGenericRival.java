package pkg;

import java.util.Iterator;
import java.util.function.Function;

public class TestVarargsGenericRival {
   interface Gene<G extends Gene<G>> {
      int value();
   }

   interface Chromosome<G extends Gene<G>> extends Iterable<G> {
      G gene();
   }

   static final class IntGene implements Gene<IntGene> {
      public int value() {
         return 1;
      }
   }

   static final class IntChromosome implements Chromosome<IntGene> {
      public IntGene gene() {
         return new IntGene();
      }

      public Iterator<IntGene> iterator() {
         return null;
      }
   }

   static final class Geno<G extends Gene<G>> {
      Chromosome<G> chromosome() {
         return null;
      }

      @SafeVarargs
      static <G extends Gene<G>> Geno<G> of(Chromosome<G> first, Chromosome<G>... rest) {
         return new Geno<>();
      }

      static <G extends Gene<G>> Geno<G> of(Iterable<? extends Chromosome<G>> chromosomes) {
         return new Geno<>();
      }
   }

   static Geno<IntGene> one(IntChromosome c) {
      return Geno.of(c);
   }

   static Function<Geno<IntGene>, Integer> decoder(IntChromosome c) {
      Geno<IntGene> g = Geno.of(c, c);
      return gt -> gt.chromosome().gene().value();
   }

   static Geno<IntGene> list(Iterable<IntChromosome> cs) {
      return Geno.of(cs);
   }
}
