package kuna.vftrace;

import java.util.concurrent.atomic.AtomicInteger;

/**
 * hashCode() of the identity-hashed Vineflower classes that gen_shadow.py patches. Each patched
 * class takes a creation index from its arena's counter (one counter per kuna arena: every
 * Statement subclass shares one, every Exprent subclass shares one, ...) and hashes by
 * {@code -Dvftrace.hash}:
 *   index        the creation index, which is what kuna's JavaHash gives the same object;
 *   random:SEED  a per-run pseudo-random value derived from SEED, the arena and the index;
 *   identity     Object.hashCode() (stock behaviour, still counting creations).
 * Counters are never reset: kuna resets its class-scope arenas only after a whole run.
 */
public final class HashMode {
  public static final String[] ARENAS = {
    "Statement", "StatEdge", "Exprent", "BasicBlock", "Instruction", "ExceptionRangeCFG", "VarVersionNode"
  };

  private static final int IDENTITY = 0;
  private static final int INDEX = 1;
  private static final int RANDOM = 2;

  private static final int MODE;
  private static final long SEED;
  private static final String SPEC;
  private static final AtomicInteger[] COUNTERS = new AtomicInteger[ARENAS.length];

  static {
    String spec = System.getProperty("vftrace.hash", "identity");
    SPEC = spec;
    if (spec.equals("index")) {
      MODE = INDEX;
      SEED = 0;
    } else if (spec.startsWith("random:")) {
      MODE = RANDOM;
      SEED = Long.parseLong(spec.substring(7));
    } else if (spec.equals("identity") || spec.equals("stock")) {
      MODE = IDENTITY;
      SEED = 0;
    } else {
      throw new IllegalArgumentException("vftrace.hash must be identity, index or random:SEED, got " + spec);
    }
    for (int i = 0; i < COUNTERS.length; i++) {
      COUNTERS[i] = new AtomicInteger();
    }
  }

  private HashMode() {
  }

  public static String describe() {
    return SPEC;
  }

  public static int next(int arena) {
    return COUNTERS[arena].getAndIncrement();
  }

  public static int hash(int arena, int index, Object self) {
    switch (MODE) {
      case INDEX:
        return index;
      case RANDOM: {
        long z = SEED * 0x9E3779B97F4A7C15L + ((long) arena << 40) + index;
        z = (z ^ (z >>> 30)) * 0xBF58476D1CE4E5B9L;
        z = (z ^ (z >>> 27)) * 0x94D049BB133111EBL;
        z = z ^ (z >>> 31);
        return (int) z;
      }
      default:
        return System.identityHashCode(self);
    }
  }
}
