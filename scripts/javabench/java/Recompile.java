import java.io.*;
import java.nio.charset.StandardCharsets;
import java.nio.file.*;
import java.util.*;
import javax.tools.*;

/**
 * javabench compile server: compiles each job's single .java file in isolation
 * (its own javac task and output dir) with a shared, cached file manager.
 *
 * usage: java Recompile JOBS.tsv RESULTS.tsv
 *   JOBS.tsv first line:  "#CP\t<classpath>"
 *   then one job per line: id \t source-file \t out-dir \t opt1 \x1f opt2 ...
 * RESULTS.tsv:
 *   J \t id \t ok(0/1) \t millis
 *   D \t id \t code \t line \t charpos \t message (tabs/newlines escaped)
 */
public class Recompile {
  static String esc(String s) {
    if (s == null) return "";
    s = s.replace("\\", "\\\\").replace("\t", "\\t").replace("\n", "\\n").replace("\r", "");
    return s.length() > 600 ? s.substring(0, 600) : s;
  }

  public static void main(String[] args) throws Exception {
    JavaCompiler jc = ToolProvider.getSystemJavaCompiler();
    List<String> lines = Files.readAllLines(Paths.get(args[0]), StandardCharsets.UTF_8);
    String cp = "";
    if (!lines.isEmpty() && lines.get(0).startsWith("#CP\t")) {
      cp = lines.get(0).substring(4);
      lines = lines.subList(1, lines.size());
    }
    StandardJavaFileManager fm = jc.getStandardFileManager(null, Locale.ROOT, StandardCharsets.UTF_8);
    List<File> cpFiles = new ArrayList<>();
    for (String p : cp.split(File.pathSeparator)) if (!p.isEmpty()) cpFiles.add(new File(p));
    fm.setLocation(StandardLocation.CLASS_PATH, cpFiles);
    File emptySp = new File(System.getProperty("javabench.sp"));
    emptySp.mkdirs();
    fm.setLocation(StandardLocation.SOURCE_PATH, List.of(emptySp));
    try (PrintWriter out = new PrintWriter(new BufferedWriter(new OutputStreamWriter(
        new FileOutputStream(args[1]), StandardCharsets.UTF_8)))) {
      for (String line : lines) {
        if (line.isBlank()) continue;
        String[] f = line.split("\t", -1);
        String id = f[0];
        File src = new File(f[1]);
        File outDir = new File(f[2]);
        outDir.mkdirs();
        List<String> opts = new ArrayList<>();
        if (f.length > 3 && !f[3].isEmpty()) opts.addAll(Arrays.asList(f[3].split("\u001f")));
        opts.add("-d");
        opts.add(outDir.getPath());
        DiagnosticCollector<JavaFileObject> dc = new DiagnosticCollector<>();
        long t0 = System.nanoTime();
        boolean ok;
        String crash = null;
        try {
          Iterable<? extends JavaFileObject> units = fm.getJavaFileObjects(src);
          ok = jc.getTask(null, fm, dc, opts, null, units).call();
        } catch (Throwable t) {
          ok = false;
          crash = t.toString();
        }
        long ms = (System.nanoTime() - t0) / 1_000_000;
        out.println("J\t" + id + "\t" + (ok ? 1 : 0) + "\t" + ms);
        if (crash != null) out.println("D\t" + id + "\tjavabench.javac.crash\t-1\t-1\t" + esc(crash));
        for (Diagnostic<? extends JavaFileObject> d : dc.getDiagnostics()) {
          if (d.getKind() != Diagnostic.Kind.ERROR) continue;
          out.println("D\t" + id + "\t" + esc(d.getCode()) + "\t" + d.getLineNumber() + "\t"
              + d.getPosition() + "\t" + esc(d.getMessage(Locale.ROOT)));
        }
        out.flush();
      }
    }
  }
}
