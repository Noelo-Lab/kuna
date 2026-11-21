package corpus;

import java.io.PrintWriter;

/** Mirrors vineflower buildSrc org.vineflower.build.JasmCompile#compile: jasm Main(PrintWriter, "jasm").compile(args). */
public final class JasmCompile {
  public static void main(String[] args) {
    PrintWriter out = new PrintWriter(System.out);
    org.openjdk.asmtools.jasm.Main jasm = new org.openjdk.asmtools.jasm.Main(out, "jasm");
    boolean ok = jasm.compile(args);
    out.flush();
    if (!ok) {
      System.err.println("jasm compile failed");
      System.exit(1);
    }
  }
}
