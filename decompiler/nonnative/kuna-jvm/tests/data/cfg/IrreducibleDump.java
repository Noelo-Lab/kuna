// Oracle for kuna's IrreducibleCFGDeobfuscator port: builds statement graphs the way
// DomHelper.graphToStatement does (basic blocks, a dummy exit, one general statement; edges to
// the first block become continues, edges to the exit breaks), optionally with nested sequence
// or general statements, and runs DomHelper.processStatement's reducibility loop: up to five
// rounds of isStatementIrreducible + splitIrreducibleNode, dumping the graph after each split.
// The graph specs are hand-written cases plus random ones from a fixed seed; each is printed
// with its result so the Rust replay needs only the dump. See gen.sh.
//
// usage: IrreducibleDump OUT

import org.jetbrains.java.decompiler.code.BytecodeVersion;
import org.jetbrains.java.decompiler.code.CodeConstants;
import org.jetbrains.java.decompiler.code.Instruction;
import org.jetbrains.java.decompiler.code.InstructionSequence;
import org.jetbrains.java.decompiler.code.cfg.BasicBlock;
import org.jetbrains.java.decompiler.main.DecompilerContext;
import org.jetbrains.java.decompiler.main.Fernflower;
import org.jetbrains.java.decompiler.main.collectors.CounterContainer;
import org.jetbrains.java.decompiler.main.decompiler.PrintStreamLogger;
import org.jetbrains.java.decompiler.main.extern.IFernflowerPreferences;
import org.jetbrains.java.decompiler.main.extern.IResultSaver;
import org.jetbrains.java.decompiler.main.plugins.PluginSources;
import org.jetbrains.java.decompiler.modules.decompiler.StatEdge;
import org.jetbrains.java.decompiler.modules.decompiler.deobfuscator.IrreducibleCFGDeobfuscator;
import org.jetbrains.java.decompiler.modules.decompiler.stats.BasicBlockStatement;
import org.jetbrains.java.decompiler.modules.decompiler.stats.DummyExitStatement;
import org.jetbrains.java.decompiler.modules.decompiler.stats.GeneralStatement;
import org.jetbrains.java.decompiler.modules.decompiler.stats.SequenceStatement;
import org.jetbrains.java.decompiler.modules.decompiler.stats.Statement;

import java.io.PrintStream;
import java.lang.reflect.Field;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.*;
import java.util.jar.Manifest;

public final class IrreducibleDump {
  private static final class NullSaver implements IResultSaver {
    public void saveFolder(String path) { }
    public void copyFile(String source, String path, String entryName) { }
    public void saveClassFile(String path, String qualifiedName, String entryName, String content, int[] mapping) { }
    public void createArchive(String path, String archiveName, Manifest manifest) { }
    public void saveDirEntry(String path, String archiveName, String entryName) { }
    public void copyEntry(String source, String path, String archiveName, String entry) { }
    public void saveClassEntry(String path, String archiveName, String qualifiedName, String entryName, String content) { }
    public void closeArchive(String path, String archiveName) { }
  }

  // node := size | '[' node ('/' node)+ ']' (sequence) | '<' node ('/' node)* '>' (general)
  private static final class Spec {
    int size;
    char kind = 'b';
    List<Spec> children = new ArrayList<>();
  }

  private static int pos;

  private static Spec parseNode(String s) {
    Spec spec = new Spec();
    char c = s.charAt(pos);
    if (c == '[' || c == '<') {
      spec.kind = c == '[' ? 's' : 'g';
      pos++;
      while (true) {
        spec.children.add(parseNode(s));
        char d = s.charAt(pos++);
        if (d == ']' || d == '>') break;
      }
    } else {
      int start = pos;
      while (pos < s.length() && Character.isDigit(s.charAt(pos))) pos++;
      spec.size = Integer.parseInt(s.substring(start, pos));
    }
    return spec;
  }

  private static Spec parseStranger() {
    Spec spec = new Spec();
    spec.size = 1;
    return spec;
  }

  private static int nextOffset;

  private static Statement build(Spec spec) {
    if (spec.kind == 'b') {
      BasicBlock block = new BasicBlock(
        DecompilerContext.getCounterContainer().getCounterAndIncrement(CounterContainer.STATEMENT_COUNTER));
      InstructionSequence seq = new InstructionSequence();
      for (int i = 0; i < spec.size; i++) {
        seq.addInstruction(Instruction.create(CodeConstants.opc_nop, false, CodeConstants.GROUP_GENERAL,
          new BytecodeVersion(52, 0), null, nextOffset++, 1));
      }
      block.setSeq(seq);
      return new BasicBlockStatement(block);
    }
    List<Statement> children = new ArrayList<>();
    for (Spec child : spec.children) {
      children.add(build(child));
    }
    Statement stat;
    if (spec.kind == 's') {
      stat = new SequenceStatement(children);
    } else {
      stat = new GeneralStatement(children.get(0), children, null);
    }
    for (int i = 0; i + 1 < children.size(); i++) {
      children.get(i).addSuccessor(new StatEdge(StatEdge.TYPE_REGULAR, children.get(i), children.get(i + 1)));
    }
    stat.setAllParent();
    return stat;
  }

  private static String id(Statement s) {
    return s == null ? "-" : Integer.toString(s.id);
  }

  private static String edges(List<StatEdge> lst) {
    if (lst.isEmpty()) return "-";
    StringBuilder sb = new StringBuilder();
    for (StatEdge e : lst) {
      if (sb.length() > 0) sb.append(',');
      sb.append(e.getType()).append(':').append(id(e.getSource())).append('>').append(id(e.getDestination()))
        .append('@').append(id(e.closure));
    }
    return sb.toString();
  }

  private static void dump(StringBuilder out, Statement s, String indent) {
    out.append("d ").append(indent).append(s.type.ordinal()).append(' ').append(s.id)
      .append(" p=").append(id(s.getParent()))
      .append(" c=").append(s.isCopied() ? 1 : 0)
      .append(" f=").append(id(s.getFirst()));
    if (s instanceof BasicBlockStatement bb) {
      out.append(" b=").append(bb.getBlock().id).append(" i=");
      InstructionSequence seq = bb.getBlock().getSeq();
      if (seq.isEmpty()) {
        out.append('-');
      }
      for (int i = 0; i < seq.length(); i++) {
        if (i > 0) out.append(',');
        out.append(seq.getInstr(i).opcode).append('@').append(seq.getInstr(i).startOffset);
      }
    }
    out.append(" S=").append(edges(s.getAllSuccessorEdges()));
    out.append(" P=").append(edges(s.getAllPredecessorEdges()));
    out.append('\n');
    for (Statement st : s.getStats()) {
      dump(out, st, indent + " ");
    }
  }

  private static String run(String nodes, String edgeSpec) throws Exception {
    Field cc = DecompilerContext.class.getDeclaredField("counterContainer");
    cc.setAccessible(true);
    cc.set(DecompilerContext.getCurrentContext(), new CounterContainer());
    nextOffset = 0;

    List<Spec> specs = new ArrayList<>();
    pos = 0;
    while (true) {
      specs.add(parseNode(nodes));
      if (pos >= nodes.length()) break;
      pos++; // ';'
    }

    StringBuilder out = new StringBuilder();
    try {
      List<Statement> stats = new ArrayList<>();
      for (Spec spec : specs) {
        stats.add(build(spec));
      }
      Statement first = stats.get(0);
      Statement exit = new DummyExitStatement();
      GeneralStatement general = new GeneralStatement(first, stats, null);
      general.setAllParent();

      Map<Integer, Integer> firstSucc = new HashMap<>();
      Statement stranger = null;
      for (String e : edgeSpec.split(",")) {
        if (e.isEmpty()) continue;
        String rest = e;
        String closureSpec = null;
        String typeSpec = null;
        int at = rest.indexOf('@');
        if (at >= 0) {
          closureSpec = rest.substring(at + 1);
          rest = rest.substring(0, at);
        }
        int colon = rest.indexOf(':');
        if (colon >= 0) {
          typeSpec = rest.substring(colon + 1);
          rest = rest.substring(0, colon);
        }
        int gt = rest.indexOf('>');
        int from = Integer.parseInt(rest.substring(0, gt));
        String toSpec = rest.substring(gt + 1);
        Statement src = stats.get(from);
        Statement dst;
        if (toSpec.equals("x")) {
          dst = exit;
        } else if (toSpec.equals("s")) {
          if (stranger == null) {
            stranger = build(parseStranger());
          }
          dst = stranger;
        } else {
          dst = stats.get(Integer.parseInt(toSpec));
        }
        if (dst != stranger) {
          firstSucc.putIfAbsent(from, toSpec.equals("x") ? -1 : Integer.parseInt(toSpec));
        }
        Statement closure = closureSpec == null ? null : closureSpec.equals("g") ? general : stats.get(Integer.parseInt(closureSpec));
        StatEdge edge;
        if (typeSpec == null) {
          if (dst == first) {
            edge = new StatEdge(StatEdge.TYPE_CONTINUE, src, general, general);
          } else if (dst == exit) {
            edge = new StatEdge(StatEdge.TYPE_BREAK, src, exit, general);
          } else {
            edge = new StatEdge(StatEdge.TYPE_REGULAR, src, dst, null);
          }
        } else if (typeSpec.equals("e")) {
          edge = new StatEdge(src, dst, List.of("java/lang/Exception"));
        } else {
          int type = switch (typeSpec) {
            case "r" -> StatEdge.TYPE_REGULAR;
            case "b" -> StatEdge.TYPE_BREAK;
            case "c" -> StatEdge.TYPE_CONTINUE;
            case "f" -> StatEdge.TYPE_FINALLYEXIT;
            default -> throw new IllegalArgumentException(typeSpec);
          };
          edge = new StatEdge(type, src, dst, closure);
        }
        src.addSuccessor(edge);
      }
      // a nested statement's last child leaves it with a labeled break to its first successor
      for (int i = 0; i < stats.size(); i++) {
        Statement stat = stats.get(i);
        Integer to = firstSucc.get(i);
        if (stat instanceof BasicBlockStatement || to == null || to == 0) continue;
        Statement last = stat.getStats().getLast();
        Statement dst = to < 0 ? exit : stats.get(to);
        last.addSuccessor(new StatEdge(StatEdge.TYPE_BREAK, last, dst, stat));
      }

      dump(out, general, "");
      dump(out, exit, "");
      for (int round = 0; round < 5; round++) {
        boolean irreducible = IrreducibleCFGDeobfuscator.isStatementIrreducible(general);
        out.append("irr ").append(irreducible ? 1 : 0).append('\n');
        if (!irreducible) break;
        boolean split = IrreducibleCFGDeobfuscator.splitIrreducibleNode(general);
        out.append("split ").append(split ? 1 : 0).append('\n');
        if (!split) break;
        dump(out, general, "");
        dump(out, exit, "");
      }
    } catch (Throwable t) {
      out.append("!error ").append(t.getClass().getSimpleName()).append(' ')
        .append(t.getMessage() == null ? "" : t.getMessage().replace("\n", "\\n")).append('\n');
    }
    return out.toString();
  }

  private static String randomNode(Random rnd, int depth) {
    int roll = rnd.nextInt(100);
    if (depth < 2 && roll < 12) {
      int n = 2 + rnd.nextInt(2);
      StringBuilder sb = new StringBuilder("[");
      for (int i = 0; i < n; i++) {
        if (i > 0) sb.append('/');
        sb.append(randomNode(rnd, depth + 1));
      }
      return sb.append(']').toString();
    }
    if (depth < 2 && roll < 14) {
      return "<" + randomNode(rnd, depth + 1) + "/" + randomNode(rnd, depth + 1) + ">";
    }
    return Integer.toString(rnd.nextInt(5));
  }

  private static String[] randomCase(Random rnd) {
    int n = 3 + rnd.nextInt(6);
    StringBuilder nodes = new StringBuilder();
    for (int i = 0; i < n; i++) {
      if (i > 0) nodes.append(';');
      nodes.append(randomNode(rnd, 0));
    }
    List<String> edges = new ArrayList<>();
    for (int i = 0; i < n; i++) {
      int out = rnd.nextInt(10) < 3 ? 1 : 2;
      if (rnd.nextInt(10) == 0) out = 3;
      Set<String> seen = new HashSet<>();
      for (int k = 0; k < out; k++) {
        int r = rnd.nextInt(n + 1);
        String to = r == n ? "x" : Integer.toString(r);
        if (!seen.add(to)) continue;
        String e = i + ">" + to;
        int special = rnd.nextInt(100);
        if (special < 3 && r < n) {
          e += ":e";
        } else if (special < 6 && r < n && r != 0) {
          e += ":b@" + rnd.nextInt(n);
        }
        edges.add(e);
      }
    }
    return new String[]{nodes.toString(), String.join(",", edges)};
  }

  public static void main(String[] args) throws Exception {
    PluginSources.PLUGIN_SOURCES.clear();
    Map<String, Object> options = new HashMap<>();
    options.put(IFernflowerPreferences.INCLUDE_JAVA_RUNTIME, "");
    options.put(IFernflowerPreferences.LOG_LEVEL, "error");
    new Fernflower(new NullSaver(), options, new PrintStreamLogger(System.err));

    List<String[]> cases = new ArrayList<>(List.of(
      new String[]{"two-entry-loop", "1;2;3", "0>1,0>2,1>2,2>1,1>x,2>x"},
      new String[]{"reducible-loop", "1;2;3", "0>1,1>2,2>1,2>x"},
      new String[]{"self-loop", "1;1", "0>1,1>1,1>x"},
      new String[]{"straight", "1;1;1", "0>1,1>2,2>x"},
      new String[]{"diamond", "1;2;2;1", "0>1,0>2,1>3,2>3,3>x"},
      new String[]{"exception-edge", "1;2;3", "0>1,0>2,1>2,2>1,1>2:e,2>x"},
      new String[]{"larger-copy-wins", "1;4;1", "0>1,0>2,1>2,2>1,1>x,2>x"},
      new String[]{"three-entry-loop", "1;1;2;3", "0>1,0>2,0>3,1>2,2>3,3>1,3>x"},
      new String[]{"nested-irreducible", "1;1;1;1;1", "0>1,0>2,1>2,2>1,2>3,3>4,4>3,1>4,4>x"},
      new String[]{"continue-to-head", "1;1;1", "0>1,0>2,1>2,2>1,1>0,2>x"},
      new String[]{"sequence-split", "1;[2/1];3", "0>1,0>2,1>2,2>1,1>x,2>x"},
      new String[]{"nested-sequence-split", "1;[1/[2/1]];5", "0>1,0>2,1>2,2>1,1>x,2>x"},
      new String[]{"closure-redirect", "1;2;3;1", "0>3,0>2,3>1,1>2,2>1,1>x,2>x,0>1:b@3"},
      new String[]{"seq-closure-redirect", "1;[2/2];1", "1>2,0>1,0>2,2>1,1>x,2>x"},
      new String[]{"general-node-copy", "1;<1/1>;3", "0>1,0>2,1>2,2>1,1>x,2>x"},
      new String[]{"outside-successor", "1;1;1", "0>1,0>2,1>2,2>1,1>x,2>x,1>s"},
      new String[]{"finally-exit-edge", "1;1;1", "0>1,0>2,1>2,2>1,1>x:f,2>x"},
      new String[]{"many-splits", "1;1;1;1;1;1", "0>1,0>2,0>3,0>4,0>5,1>2,2>3,3>4,4>5,5>1,5>x"}
    ));
    Random rnd = new Random(20261109L);
    for (int i = 0; i < 400; i++) {
      String[] c = randomCase(rnd);
      cases.add(new String[]{"random-" + i, c[0], c[1]});
    }

    try (PrintStream ps = new PrintStream(Files.newOutputStream(Paths.get(args[0])), false, StandardCharsets.UTF_8)) {
      ps.append("#irreducible vineflower-1.12.0\n");
      int irreducible = 0;
      for (String[] c : cases) {
        String body = run(c[1], c[2]);
        if (body.contains("irr 1")) irreducible++;
        ps.append("=case ").append(c[0]).append(' ').append(c[1]).append(' ').append(c[2]).append('\n');
        ps.append(body);
        ps.append("=end\n");
      }
      System.err.println("cases: " + cases.size() + ", irreducible at least once: " + irreducible);
    }
  }
}
