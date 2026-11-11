// Oracle for kuna's CFG port: runs Vineflower 1.12.0's ControlFlowGraph construction and the
// DeadCodeHelper/ExceptionDeobfuscator steps MethodProcessor.codeToJava runs before
// DomHelper.parseGraph, and dumps each method's input instructions plus the graph after
// construction ("built"), after goto removal ("pre") and before statement parsing ("final").
// Run with patch/ (the deterministic-hash BasicBlock) ahead of the Vineflower jar; see gen.sh.
//
// usage: CfgDump OUT [--max-insns N] [--skip-trivial] INPUT...
//        INPUT = file.class | dir | jar | jrt:module/pkg/Cls.class
// --skip-trivial drops methods without branches or handlers (one block after construction).

import org.jetbrains.java.decompiler.code.CodeConstants;
import org.jetbrains.java.decompiler.code.ExceptionHandler;
import org.jetbrains.java.decompiler.code.FullInstructionSequence;
import org.jetbrains.java.decompiler.code.Instruction;
import org.jetbrains.java.decompiler.code.cfg.BasicBlock;
import org.jetbrains.java.decompiler.code.cfg.ControlFlowGraph;
import org.jetbrains.java.decompiler.code.cfg.ExceptionRangeCFG;
import org.jetbrains.java.decompiler.main.DecompilerContext;
import org.jetbrains.java.decompiler.main.Fernflower;
import org.jetbrains.java.decompiler.main.decompiler.PrintStreamLogger;
import org.jetbrains.java.decompiler.main.extern.IFernflowerPreferences;
import org.jetbrains.java.decompiler.main.extern.IResultSaver;
import org.jetbrains.java.decompiler.main.plugins.PluginSources;
import org.jetbrains.java.decompiler.modules.code.DeadCodeHelper;
import org.jetbrains.java.decompiler.modules.code.ExceptionDeobfuscator;
import org.jetbrains.java.decompiler.modules.decompiler.decompose.GenericDominatorEngine;
import org.jetbrains.java.decompiler.modules.decompiler.decompose.IGraph;
import org.jetbrains.java.decompiler.modules.decompiler.decompose.IGraphNode;
import org.jetbrains.java.decompiler.struct.StructClass;
import org.jetbrains.java.decompiler.struct.StructMethod;
import org.jetbrains.java.decompiler.util.DataInputFullStream;
import org.jetbrains.java.decompiler.util.collections.VBStyleCollection;

import java.io.PrintStream;
import java.lang.reflect.Field;
import java.net.URI;
import java.nio.charset.StandardCharsets;
import java.nio.file.FileSystem;
import java.nio.file.FileSystems;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.*;
import java.util.jar.Manifest;
import java.util.stream.Collectors;
import java.util.stream.Stream;

public final class CfgDump {
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

  private static String ids(Collection<BasicBlock> blocks) {
    if (blocks.isEmpty()) return "-";
    return blocks.stream().map(b -> b == null ? "null" : Integer.toString(b.id)).collect(Collectors.joining(","));
  }

  private static String ints(Collection<Integer> values) {
    if (values.isEmpty()) return "-";
    return values.stream().map(String::valueOf).collect(Collectors.joining(","));
  }

  private static void dumpGraph(StringBuilder out, String stage, ControlFlowGraph graph) {
    out.append('@').append(stage).append('\n');
    for (BasicBlock block : graph.getBlocks()) {
      out.append("b ").append(block.id).append(" s=");
      if (block.getSeq().isEmpty()) {
        out.append('-');
      } else {
        for (int i = 0; i < block.getSeq().length(); i++) {
          Instruction instr = block.getSeq().getInstr(i);
          if (i > 0) out.append(',');
          out.append(instr.opcode).append('@').append(instr.startOffset);
        }
      }
      out.append(" o=").append(ints(block.getInstrOldOffsets()));
      out.append(" S=").append(ids(block.getSuccs()));
      out.append(" P=").append(ids(block.getPreds()));
      out.append(" E=").append(ids(block.getSuccExceptions()));
      out.append(" Q=").append(ids(block.getPredExceptions()));
      out.append(" m=").append(block.mark);
      out.append('\n');
    }
    out.append("f ").append(graph.getFirst().id)
      .append(" l ").append(graph.getLast().id)
      .append(" L=").append(ids(graph.getLast().getPreds()))
      .append(" LS=").append(ids(graph.getLast().getSuccs()))
      .append(" n=").append(graph.last_id).append('\n');
    for (ExceptionRangeCFG range : graph.getExceptions()) {
      List<String> types = range.getExceptionTypes();
      out.append("r ").append(range.getHandler() == null ? "null" : Integer.toString(range.getHandler().id))
        .append(" B=").append(ids(range.getProtectedRange()))
        .append(" T=").append(types == null ? "null" : types.isEmpty() ? "-" : String.join("|", types))
        .append('\n');
    }
    out.append("F=").append(ids(graph.getFinallyExits())).append('\n');
  }

  @SuppressWarnings("unchecked")
  private static void dumpDominators(StringBuilder out, ControlFlowGraph graph) throws Exception {
    List<BasicBlock> rpo = graph.getReversePostOrder();
    out.append("rpo ").append(ids(rpo)).append('\n');

    GenericDominatorEngine engine = new GenericDominatorEngine(new IGraph() {
      @Override
      public List<? extends IGraphNode> getReversePostOrderList() {
        return graph.getReversePostOrder();
      }

      @Override
      public Set<? extends IGraphNode> getRoots() {
        return new HashSet<>(Collections.singletonList(graph.getFirst()));
      }
    });
    engine.initialize();

    Field f = GenericDominatorEngine.class.getDeclaredField("colOrderedIDoms");
    f.setAccessible(true);
    VBStyleCollection<IGraphNode, IGraphNode> idoms = (VBStyleCollection<IGraphNode, IGraphNode>) f.get(engine);
    out.append("idom");
    for (BasicBlock block : rpo) {
      BasicBlock idom = (BasicBlock) idoms.getWithKey(block);
      out.append(' ').append(block.id).append('=').append(idom == null ? "null" : Integer.toString(idom.id));
    }
    out.append('\n');
  }

  private static boolean needsPool(ControlFlowGraph graph) {
    if (DecompilerContext.getOption(IFernflowerPreferences.OLD_TRY_DEDUP)) return false;
    Map<BasicBlock, Integer> counts = new HashMap<>();
    for (ExceptionRangeCFG range : graph.getExceptions()) {
      counts.merge(range.getHandler(), 1, Integer::sum);
    }
    for (Map.Entry<BasicBlock, Integer> e : counts.entrySet()) {
      if (e.getValue() > 1) {
        for (Instruction instr : e.getKey().getSeq()) {
          if (instr.opcode == CodeConstants.opc_new) return true;
        }
      }
    }
    return false;
  }

  private static String oneLine(Throwable t) {
    String msg = t.getMessage() == null ? "" : t.getMessage();
    return t.getClass().getSimpleName() + " " + msg.replace("\n", "\\n");
  }

  private static boolean isTrivial(FullInstructionSequence seq) {
    if (!seq.exceptionTable().getHandlers().isEmpty()) return false;
    for (int i = 0; i < seq.length(); i++) {
      int group = seq.getInstr(i).group;
      if (group == CodeConstants.GROUP_JUMP || group == CodeConstants.GROUP_SWITCH) return false;
      if (group == CodeConstants.GROUP_RETURN && i + 1 < seq.length()) return false;
    }
    return true;
  }

  private static void processMethod(PrintStream ps, StructClass cl, StructMethod mt, int maxInsns, boolean skipTrivial, int[] stats) throws Exception {
    mt.expandData(cl);
    FullInstructionSequence seq = mt.getInstructionSequence();
    if (seq.length() > maxInsns || (skipTrivial && isTrivial(seq))) {
      stats[1]++;
      return;
    }

    StringBuilder out = new StringBuilder();
    for (int i = 0; i < seq.length(); i++) {
      Instruction instr = seq.getInstr(i);
      out.append("i ").append(i).append(' ').append(instr.startOffset).append(' ').append(instr.length)
        .append(' ').append(instr.opcode).append(' ').append(instr.group).append(' ').append(instr.wide ? 1 : 0).append(' ');
      if (instr.operandsCount() == 0) {
        out.append('-');
      } else {
        for (int j = 0; j < instr.operandsCount(); j++) {
          if (j > 0) out.append(',');
          out.append(instr.operand(j));
        }
      }
      out.append('\n');
    }
    for (ExceptionHandler h : seq.exceptionTable().getHandlers()) {
      out.append("x ").append(h.from()).append(' ').append(h.to()).append(' ').append(h.handler()).append(' ')
        .append(h.exceptionClass() == null ? "-" : h.exceptionClass()).append('\n');
    }

    boolean jsr = mt.getBytecodeVersion().hasJsr() || DecompilerContext.getOption(IFernflowerPreferences.FORCE_JSR_INLINE);
    boolean pool = false;
    String stage = "built";
    BasicBlock.resetIdentity();
    try {
      ControlFlowGraph graph = new ControlFlowGraph(seq);
      dumpGraph(out, "built", graph);

      stage = "pre";
      DeadCodeHelper.removeDeadBlocks(graph);
      if (jsr) {
        graph.inlineJsr(cl, mt);
      }
      DeadCodeHelper.connectDummyExitBlock(graph);
      DeadCodeHelper.removeGotos(graph);
      dumpGraph(out, "pre", graph);

      stage = "final";
      ExceptionDeobfuscator.removeCircularRanges(graph);
      ExceptionDeobfuscator.restorePopRanges(graph);
      if (DecompilerContext.getOption(IFernflowerPreferences.REMOVE_EMPTY_RANGES)) {
        ExceptionDeobfuscator.removeEmptyRanges(graph);
      }
      if (DecompilerContext.getOption(IFernflowerPreferences.ENSURE_SYNCHRONIZED_MONITOR)) {
        DeadCodeHelper.extendSynchronizedRangeToMonitorexit(graph);
      }
      if (DecompilerContext.getOption(IFernflowerPreferences.INCORPORATE_RETURNS)) {
        DeadCodeHelper.incorporateValueReturns(graph);
      }
      ExceptionDeobfuscator.insertEmptyExceptionHandlerBlocks(graph);
      DeadCodeHelper.mergeBasicBlocks(graph);
      if (ExceptionDeobfuscator.hasObfuscatedExceptions(graph)) {
        if (!ExceptionDeobfuscator.handleMultipleEntryExceptionRanges(graph)) {
          graph.addComment("$VF: Could not handle exception ranges with multiple entries");
          graph.addErrorComment = true;
        }
        pool = needsPool(graph);
        ExceptionDeobfuscator.insertDummyExceptionHandlerBlocks(graph, mt.getBytecodeVersion());
      }
      dumpGraph(out, "final", graph);
      if (graph.commentLines != null) {
        for (String c : graph.commentLines) {
          out.append("c ").append(c).append('\n');
        }
      }
      out.append("e ").append(graph.addErrorComment ? 1 : 0).append('\n');

      stage = "dom";
      dumpDominators(out, graph);
    }
    catch (Throwable t) {
      out.append("!error ").append(stage).append(' ').append(oneLine(t)).append('\n');
      stats[2]++;
    }

    ps.append("=method ").append(cl.qualifiedName).append(' ').append(mt.getName()).append(' ').append(mt.getDescriptor())
      .append(" v=").append(Integer.toString(mt.getBytecodeVersion().major)).append('.')
      .append(Integer.toString(mt.getBytecodeVersion().minor))
      .append(" jsr=").append(jsr ? "1" : "0").append(" pool=").append(pool ? "1" : "0").append('\n');
    ps.append(out);
    ps.append("=end\n");
    stats[0]++;
  }

  private static List<byte[]> collect(String input) throws Exception {
    List<byte[]> res = new ArrayList<>();
    if (input.startsWith("jrt:")) {
      FileSystem jrt = FileSystems.getFileSystem(URI.create("jrt:/"));
      res.add(Files.readAllBytes(jrt.getPath("modules", input.substring(4))));
      return res;
    }
    Path p = Paths.get(input);
    if (input.endsWith(".jar")) {
      try (java.util.zip.ZipFile zip = new java.util.zip.ZipFile(p.toFile())) {
        List<? extends java.util.zip.ZipEntry> entries = Collections.list(zip.entries());
        entries.sort(Comparator.comparing(java.util.zip.ZipEntry::getName));
        for (java.util.zip.ZipEntry e : entries) {
          if (e.getName().endsWith(".class")) res.add(zip.getInputStream(e).readAllBytes());
        }
      }
    } else if (Files.isDirectory(p)) {
      List<Path> files;
      try (Stream<Path> s = Files.walk(p)) {
        files = s.filter(f -> f.toString().endsWith(".class")).sorted().collect(Collectors.toList());
      }
      for (Path f : files) res.add(Files.readAllBytes(f));
    } else {
      res.add(Files.readAllBytes(p));
    }
    return res;
  }

  public static void main(String[] args) throws Exception {
    PluginSources.PLUGIN_SOURCES.clear();
    Map<String, Object> options = new HashMap<>();
    options.put(IFernflowerPreferences.INCLUDE_JAVA_RUNTIME, "");
    options.put(IFernflowerPreferences.LOG_LEVEL, "error");
    new Fernflower(new NullSaver(), options, new PrintStreamLogger(System.err));

    String outPath = args[0];
    int maxInsns = Integer.MAX_VALUE;
    boolean skipTrivial = false;
    int k = 1;
    while (k < args.length && args[k].startsWith("--")) {
      if (args[k].equals("--max-insns")) {
        maxInsns = Integer.parseInt(args[k + 1]);
        k += 2;
      } else if (args[k].equals("--skip-trivial")) {
        skipTrivial = true;
        k++;
      } else {
        throw new IllegalArgumentException("unknown flag " + args[k]);
      }
    }

    int[] stats = new int[3];
    try (PrintStream ps = new PrintStream(Files.newOutputStream(Paths.get(outPath)), false, StandardCharsets.UTF_8)) {
      ps.append("#cfgdump vineflower-1.12.0 options: remove-empty-try-catch=")
        .append(DecompilerContext.getOption(IFernflowerPreferences.REMOVE_EMPTY_RANGES) ? "1" : "0")
        .append(" ensure-synchronized-monitors=").append(DecompilerContext.getOption(IFernflowerPreferences.ENSURE_SYNCHRONIZED_MONITOR) ? "1" : "0")
        .append(" incorporate-returns=").append(DecompilerContext.getOption(IFernflowerPreferences.INCORPORATE_RETURNS) ? "1" : "0")
        .append(" old-try-dedup=").append(DecompilerContext.getOption(IFernflowerPreferences.OLD_TRY_DEDUP) ? "1" : "0")
        .append(" force-jsr-inline=").append(DecompilerContext.getOption(IFernflowerPreferences.FORCE_JSR_INLINE) ? "1" : "0")
        .append('\n');
      for (int a = k; a < args.length; a++) {
        for (byte[] bytes : collect(args[a])) {
          StructClass cl = StructClass.create(new DataInputFullStream(bytes), true);
          DecompilerContext.setProperty(DecompilerContext.CURRENT_CLASS, cl);
          for (StructMethod mt : cl.getMethods()) {
            if (mt.containsCode()) {
              processMethod(ps, cl, mt, maxInsns, skipTrivial, stats);
            }
          }
          DecompilerContext.setProperty(DecompilerContext.CURRENT_CLASS, null);
        }
      }
    }
    System.err.println("methods dumped: " + stats[0] + ", skipped (size/trivial): " + stats[1] + ", with exception: " + stats[2]);
  }
}
