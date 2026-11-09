// Oracle for tests/b08_lift.rs: runs Vineflower 1.12.0's ExprProcessor.processBlock on every
// basic block of the raw ControlFlowGraph of each method (each block on an empty stack, as
// processStatement starts the first one) and dumps the expressions it builds, with their ids,
// types and bytecode offsets, plus the counters at the end of each method. Each class runs
// as a root ClassNode with a fresh ImportCollector. See gen.sh.

import org.jetbrains.java.decompiler.code.cfg.BasicBlock;
import org.jetbrains.java.decompiler.code.cfg.ControlFlowGraph;
import org.jetbrains.java.decompiler.main.ClassesProcessor.ClassNode;
import org.jetbrains.java.decompiler.main.DecompilerContext;
import org.jetbrains.java.decompiler.main.Fernflower;
import org.jetbrains.java.decompiler.main.collectors.CounterContainer;
import org.jetbrains.java.decompiler.main.collectors.ImportCollector;
import org.jetbrains.java.decompiler.main.decompiler.PrintStreamLogger;
import org.jetbrains.java.decompiler.main.extern.IFernflowerPreferences;
import org.jetbrains.java.decompiler.main.extern.IResultSaver;
import org.jetbrains.java.decompiler.main.plugins.PluginSources;
import org.jetbrains.java.decompiler.modules.decompiler.ExprProcessor;
import org.jetbrains.java.decompiler.modules.decompiler.PrimitiveExprsList;
import org.jetbrains.java.decompiler.modules.decompiler.exps.*;
import org.jetbrains.java.decompiler.modules.decompiler.stats.BasicBlockStatement;
import org.jetbrains.java.decompiler.modules.decompiler.vars.VarProcessor;
import org.jetbrains.java.decompiler.struct.StructClass;
import org.jetbrains.java.decompiler.struct.StructMethod;
import org.jetbrains.java.decompiler.struct.gen.MethodDescriptor;
import org.jetbrains.java.decompiler.struct.gen.VarType;
import org.jetbrains.java.decompiler.util.DataInputFullStream;

import java.io.PrintStream;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.*;
import java.util.jar.Manifest;

public final class GenB08Oracle {
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

  static String t(VarType v) {
    return v == null ? "null" : v.toString();
  }

  static String e(Exprent x) {
    if (x == null) {
      return "null";
    }
    StringBuilder sb = new StringBuilder();
    sb.append(x.type).append('#').append(x.id).append(':').append(t(x.getExprType()));
    if (x.bytecode != null) {
      sb.append(x.bytecode);
    }
    sb.append('[');
    if (x instanceof VarExprent v) {
      sb.append(v.getIndex()).append(v.isStack() ? " stack" : "");
      if (v.getLVT() != null) {
        sb.append(" lvt=").append(v.getLVT().getName());
      }
      if (v.isCatchTempVar()) {
        sb.append(" catch");
      }
    } else if (x instanceof ConstExprent c) {
      Object val = c.getValue();
      sb.append(val == null ? "null" : val.getClass().getSimpleName() + "=" + val);
      if (c.isBoolPermitted()) {
        sb.append(" bool");
      }
    } else if (x instanceof AssignmentExprent a) {
      sb.append(e(a.getLeft())).append(" = ").append(e(a.getRight()));
    } else if (x instanceof FunctionExprent f) {
      sb.append(f.getFuncType());
      for (Exprent op : f.getLstOperands()) {
        sb.append(' ').append(e(op));
      }
    } else if (x instanceof ArrayExprent a) {
      sb.append(e(a.getArray())).append(' ').append(e(a.getIndex()));
    } else if (x instanceof FieldExprent f) {
      sb.append(f.getClassname()).append('.').append(f.getName()).append(' ').append(f.getDescriptor().descriptorString);
      sb.append(f.isStatic() ? " static" : " " + e(f.getInstance()));
    } else if (x instanceof IfExprent i) {
      sb.append(e(i.getCondition()));
    } else if (x instanceof ExitExprent ex) {
      sb.append(ex.getExitType()).append(' ').append(e(ex.getValue())).append(' ').append(t(ex.getRetType()));
    } else if (x instanceof MonitorExprent m) {
      sb.append(m.getMonType()).append(' ').append(e(m.getValue())).append(m.isRemovable() ? " remove" : "");
    } else if (x instanceof NewExprent n) {
      sb.append(t(n.getNewType()));
      for (Exprent d : n.getLstDims()) {
        sb.append(' ').append(e(d));
      }
    } else if (x instanceof SwitchHeadExprent s) {
      sb.append(e(s.getValue()));
    } else if (x instanceof InvocationExprent i) {
      sb.append(i.getClassname()).append('.').append(i.getName()).append(i.getStringDescriptor());
      sb.append(i.isStatic() ? " static" : " " + e(i.getInstance()));
      for (Exprent p : i.getLstParameters()) {
        sb.append(' ').append(e(p));
      }
    }
    return sb.append(']').toString();
  }

  public static void main(String[] args) throws Exception {
    PluginSources.PLUGIN_SOURCES.clear();
    Map<String, Object> options = new HashMap<>();
    options.put(IFernflowerPreferences.INCLUDE_JAVA_RUNTIME, "");
    options.put(IFernflowerPreferences.LOG_LEVEL, "error");
    new Fernflower(new NullSaver(), options, new PrintStreamLogger(System.err));

    try (PrintStream ps = new PrintStream(Files.newOutputStream(Paths.get(args[0])), false, StandardCharsets.UTF_8)) {
      for (int a = 1; a < args.length; a++) {
        byte[] bytes = Files.readAllBytes(Paths.get(args[a]));
        StructClass cl = StructClass.create(new DataInputFullStream(bytes), true);
        DecompilerContext.setProperty(DecompilerContext.CURRENT_CLASS, cl);
        ClassNode node = new ClassNode(ClassNode.Type.ROOT, cl);
        DecompilerContext.setProperty(DecompilerContext.CURRENT_CLASS_NODE, node);
        DecompilerContext.startClass(new ImportCollector(node));
        for (StructMethod mt : cl.getMethods()) {
          if (!mt.containsCode()) {
            continue;
          }
          ps.append("M ").append(cl.qualifiedName).append('.').append(mt.getName()).append(mt.getDescriptor()).append('\n');
          mt.expandData(cl);
          MethodDescriptor md = MethodDescriptor.parseDescriptor(mt, null);
          VarProcessor varProc = new VarProcessor(mt, md);
          DecompilerContext.startMethod(varProc);
          ControlFlowGraph graph = new ControlFlowGraph(mt.getInstructionSequence());
          ExprProcessor proc = new ExprProcessor(md, varProc);
          for (BasicBlock block : graph.getBlocks()) {
            BasicBlockStatement stat = new BasicBlockStatement(block);
            PrimitiveExprsList data = new PrimitiveExprsList();
            ps.append("B ").append(String.valueOf(block.id)).append(" stat ").append(String.valueOf(stat.id)).append('\n');
            try {
              proc.processBlock(stat, data, cl);
            } catch (Throwable ex) {
              ps.append("  EXC\n");
            }
            for (Exprent x : data.getLstExprents()) {
              ps.append("  ").append(e(x)).append('\n');
            }
            for (Exprent x : data.getStack()) {
              ps.append("  S ").append(e(x)).append('\n');
            }
          }
          CounterContainer counters = DecompilerContext.getCounterContainer();
          ps.append("C expr=").append(String.valueOf(counters.getCounter(CounterContainer.EXPRESSION_COUNTER)))
            .append(" stat=").append(String.valueOf(counters.getCounter(CounterContainer.STATEMENT_COUNTER)))
            .append('\n');
          mt.releaseResources();
        }
        DecompilerContext.setProperty(DecompilerContext.CURRENT_CLASS_NODE, null);
        DecompilerContext.setProperty(DecompilerContext.CURRENT_CLASS, null);
      }
    }
  }
}
