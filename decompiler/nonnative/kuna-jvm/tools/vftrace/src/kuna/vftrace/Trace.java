package kuna.vftrace;

import org.jetbrains.java.decompiler.modules.decompiler.stats.RootStatement;
import org.jetbrains.java.decompiler.struct.StructMethod;

import java.io.BufferedWriter;
import java.io.IOException;
import java.io.UncheckedIOException;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.HashMap;
import java.util.Map;

/**
 * Streams one canonical dump per DecompileRecord.add(name, root) to the trace file, exactly as kuna's
 * src/infra/kuna_trace.rs does: a "=M class#name+descriptor" line whenever the method changes, then
 * "@ n Name [#k]" (n counts this method's steps across all of its records, k repeats of the same name)
 * and the dump, or "= n-1" when the tree is unchanged since the method's previous step.
 */
public final class Trace {
  private static final class MethodState {
    int steps;
    final Map<String, Integer> repeats = new HashMap<>();
    String last;
  }

  private static volatile boolean enabled = false;
  private static String filter;
  private static BufferedWriter out;
  private static String lastKey;
  private static final Map<String, MethodState> METHODS = new HashMap<>();
  private static long steps = 0;
  private static long dumpNanos = 0;
  private static long dumpChars = 0;

  private Trace() {
  }

  public static void start(Path path, String header, String methodFilter) throws IOException {
    Dumper.init();
    Path parent = path.toAbsolutePath().getParent();
    if (parent != null) {
      Files.createDirectories(parent);
    }
    out = Files.newBufferedWriter(path, StandardCharsets.UTF_8);
    out.write("#vftrace 1 " + header + "\n");
    filter = methodFilter;
    enabled = true;
  }

  public static void step(StructMethod mt, String name, RootStatement root) {
    if (!enabled) {
      return;
    }
    String key = mt.getClassQualifiedName() + "#" + mt.getName() + mt.getDescriptor();
    if (filter != null && !key.contains(filter)) {
      return;
    }
    long t0 = System.nanoTime();
    String dump;
    try {
      dump = Dumper.dump(root);
    } catch (RuntimeException e) {
      dump = "! dump failed: " + e + "\n";
    }
    long t1 = System.nanoTime();
    synchronized (Trace.class) {
      dumpNanos += t1 - t0;
      dumpChars += dump.length();
      StringBuilder text = new StringBuilder();
      if (!key.equals(lastKey)) {
        text.append("=M ").append(key).append('\n');
        lastKey = key;
      }
      MethodState m = METHODS.computeIfAbsent(key, k -> new MethodState());
      int repeat = m.repeats.merge(name, 1, Integer::sum);
      int n = m.steps++;
      text.append("@ ").append(n).append(' ').append(name);
      if (repeat > 1) {
        text.append(" #").append(repeat);
      }
      text.append('\n');
      if (dump.equals(m.last)) {
        text.append("= ").append(n - 1).append('\n');
      } else {
        text.append(dump);
        m.last = dump;
      }
      steps++;
      try {
        out.write(text.toString());
      } catch (IOException e) {
        throw new UncheckedIOException(e);
      }
    }
  }

  /** Stops tracing, closes the file and returns a one-line summary. */
  public static synchronized String finish() throws IOException {
    enabled = false;
    out.close();
    return METHODS.size() + " methods, " + steps + " steps, dump " + (dumpNanos / 1_000_000) + " ms, "
      + dumpChars + " chars";
  }
}
