// Oracle for kuna's b11_finally test: for every method with code of every input class, runs
// MethodProcessor.codeToJava up to the first DomHelper.parseGraph, then
// FinallyProcessor.iterateGraph, and prints what it returned or threw. See gen.sh.
//
// usage: FinallyDump OUT INPUT...      INPUT = file.class | dir (classes in path order)

import org.jetbrains.java.decompiler.code.cfg.ControlFlowGraph;
import org.jetbrains.java.decompiler.main.DecompilerContext;
import org.jetbrains.java.decompiler.main.Fernflower;
import org.jetbrains.java.decompiler.main.decompiler.PrintStreamLogger;
import org.jetbrains.java.decompiler.main.extern.IFernflowerPreferences;
import org.jetbrains.java.decompiler.main.extern.IResultSaver;
import org.jetbrains.java.decompiler.main.plugins.PluginSources;
import org.jetbrains.java.decompiler.main.collectors.CounterContainer;
import org.jetbrains.java.decompiler.modules.code.DeadCodeHelper;
import org.jetbrains.java.decompiler.modules.decompiler.FinallyProcessor;
import org.jetbrains.java.decompiler.modules.code.ExceptionDeobfuscator;
import org.jetbrains.java.decompiler.modules.decompiler.decompose.DomHelper;
import org.jetbrains.java.decompiler.modules.decompiler.stats.RootStatement;
import org.jetbrains.java.decompiler.modules.decompiler.vars.VarProcessor;
import org.jetbrains.java.decompiler.struct.StructClass;
import org.jetbrains.java.decompiler.struct.StructMethod;
import org.jetbrains.java.decompiler.struct.gen.MethodDescriptor;
import org.jetbrains.java.decompiler.util.DataInputFullStream;

import java.io.PrintStream;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.*;
import java.util.jar.Manifest;
import java.util.stream.Collectors;
import java.util.stream.Stream;

public final class FinallyDump {
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

  private static List<Path> collect(String input) throws Exception {
    Path p = Paths.get(input);
    if (Files.isDirectory(p)) {
      try (Stream<Path> s = Files.list(p)) {
        return s.filter(f -> f.toString().endsWith(".class")).sorted().collect(Collectors.toList());
      }
    }
    return List.of(p);
  }

  private static Object[] parse(StructClass cl, StructMethod mt) throws Exception {
    mt.expandData(cl);
    ControlFlowGraph graph = new ControlFlowGraph(mt.getInstructionSequence());
    DecompilerContext.getCounterContainer().setCounter(CounterContainer.VAR_COUNTER, mt.getLocalVariables());
    DeadCodeHelper.removeDeadBlocks(graph);
    if (mt.getBytecodeVersion().hasJsr() || DecompilerContext.getOption(IFernflowerPreferences.FORCE_JSR_INLINE)) {
      graph.inlineJsr(cl, mt);
    }
    DeadCodeHelper.connectDummyExitBlock(graph);
    DeadCodeHelper.removeGotos(graph);
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
      ExceptionDeobfuscator.insertDummyExceptionHandlerBlocks(graph, mt.getBytecodeVersion());
    }
    return new Object[]{graph, DomHelper.parseGraph(graph, mt, 0)};
  }

  public static void main(String[] args) throws Exception {
    PluginSources.PLUGIN_SOURCES.clear();
    Map<String, Object> options = new HashMap<>();
    options.put(IFernflowerPreferences.INCLUDE_JAVA_RUNTIME, "");
    options.put(IFernflowerPreferences.LOG_LEVEL, "error");
    new Fernflower(new NullSaver(), options, new PrintStreamLogger(System.err));

    try (PrintStream ps = new PrintStream(Files.newOutputStream(Paths.get(args[0])), false, StandardCharsets.UTF_8)) {
      ps.append("#finally vineflower-1.12.0\n");
      for (int a = 1; a < args.length; a++) {
        for (Path file : collect(args[a])) {
          StructClass cl = StructClass.create(new DataInputFullStream(Files.readAllBytes(file)), true);
          DecompilerContext.setProperty(DecompilerContext.CURRENT_CLASS, cl);
          for (StructMethod mt : cl.getMethods()) {
            if (!mt.containsCode()) {
              continue;
            }
            MethodDescriptor md = MethodDescriptor.parseDescriptor(mt.getDescriptor());
            VarProcessor varProc = new VarProcessor(mt, md);
            DecompilerContext.startMethod(varProc);
            String result;
            Object[] parsed;
            try {
              parsed = parse(cl, mt);
            }
            catch (Throwable t) {
              parsed = null;
            }
            if (parsed == null) {
              result = "unparsed";
            }
            else {
              try {
                result = String.valueOf(new FinallyProcessor(mt, md, varProc)
                  .iterateGraph(cl, mt, (RootStatement)parsed[1], (ControlFlowGraph)parsed[0]));
              }
              catch (Throwable t) {
                result = "throws " + t.getClass().getSimpleName();
              }
            }
            ps.append(cl.qualifiedName).append('.').append(mt.getName()).append(mt.getDescriptor())
              .append('\t').append(result).append('\n');
          }
          DecompilerContext.setProperty(DecompilerContext.CURRENT_CLASS, null);
        }
      }
    }
  }
}
