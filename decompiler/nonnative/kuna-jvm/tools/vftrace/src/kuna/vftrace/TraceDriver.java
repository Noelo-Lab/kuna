package kuna.vftrace;

import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.List;

/**
 * Runs corpus.VfDriver (the corpus oracle's option handling, unchanged) with the pass tracer on.
 *
 * usage: TraceDriver --trace OUT [--filter SUBSTRING] [--shift K] VFDRIVER-ARGS...
 *   --trace OUT     trace file to write
 *   --filter S      only trace methods whose key (class#name+descriptor) contains S
 *   --shift K       start K throwaway threads first: each consumes one os::random() draw, so the
 *                   decompiler thread's identity-hash sequence starts elsewhere (stock-hash perturbation)
 * VfDriver arguments follow (--fixture, --opt k=v, ..., inputs, -o OUT); "-o" defaults to a fresh
 * directory next to the trace. "--opt thread-count=1" is prepended so that all classes are processed on
 * one worker thread (identity hashes and creation counters then depend on nothing but the input); a
 * later --opt thread-count=N overrides it.
 */
public final class TraceDriver {
  public static void main(String[] args) throws Exception {
    String trace = null;
    String filter = null;
    int shift = 0;
    List<String> rest = new ArrayList<>();
    rest.add("--opt");
    rest.add("thread-count=1");
    boolean hasOut = false;
    for (int i = 0; i < args.length; i++) {
      String a = args[i];
      if (a.equals("--trace") && i + 1 < args.length) {
        trace = args[++i];
      } else if (a.equals("--filter") && i + 1 < args.length) {
        filter = args[++i];
      } else if (a.equals("--shift") && i + 1 < args.length) {
        shift = Integer.parseInt(args[++i]);
      } else {
        if (a.equals("-o")) hasOut = true;
        rest.add(a);
      }
    }
    if (trace == null) {
      System.err.println("usage: TraceDriver --trace OUT [--filter S] [--shift K] VFDRIVER-ARGS...");
      System.exit(64);
    }
    Path tracePath = Paths.get(trace).toAbsolutePath();
    if (!hasOut) {
      Path parent = tracePath.getParent();
      Files.createDirectories(parent);
      rest.add("-o");
      rest.add(Files.createTempDirectory(parent, tracePath.getFileName() + ".out.").toString());
    }
    for (int i = 0; i < shift; i++) {
      Thread t = new Thread(() -> { });
      t.start();
      t.join();
    }
    Trace.start(tracePath, "engine=vineflower hash=" + HashMode.describe() + " shift=" + shift, filter);
    long t0 = System.nanoTime();
    corpus.VfDriver.main(rest.toArray(new String[0]));
    long ms = (System.nanoTime() - t0) / 1_000_000;
    String summary = Trace.finish();
    System.err.println("vftrace: " + summary + ", run " + ms + " ms -> " + tracePath);
  }
}
