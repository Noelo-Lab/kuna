// Golden vectors for tests/b07a_var_definition.rs: the real Vineflower 1.12.0
// VarDefinitionHelper.remapClashingNames on hand-built statement trees over the
// K2Shapes.straight(II)I fixture method. Regenerate with `sh gen.sh <vineflower-1.12.0.jar>`.
// Each line is `<scenario> <var>,<version>=<name> ...` in getClashingNames() iteration order.

import java.lang.reflect.Field;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.*;

import org.jetbrains.java.decompiler.main.ClassesProcessor;
import org.jetbrains.java.decompiler.main.DecompilerContext;
import org.jetbrains.java.decompiler.main.collectors.CounterContainer;
import org.jetbrains.java.decompiler.main.extern.IFernflowerPreferences;
import org.jetbrains.java.decompiler.modules.decompiler.exps.*;
import org.jetbrains.java.decompiler.modules.decompiler.stats.*;
import org.jetbrains.java.decompiler.modules.decompiler.vars.*;
import org.jetbrains.java.decompiler.struct.StructClass;
import org.jetbrains.java.decompiler.struct.StructMethod;
import org.jetbrains.java.decompiler.struct.gen.VarType;
import org.jetbrains.java.decompiler.util.DataInputFullStream;

public class VarDefOracle {
  static StructMethod mt;
  static VarProcessor varproc;

  static void setField(Object o, Class<?> cls, String name, Object value) throws Exception {
    Field f = cls.getDeclaredField(name);
    f.setAccessible(true);
    f.set(o, value);
  }

  static void install() throws Exception {
    Field uf = sun.misc.Unsafe.class.getDeclaredField("theUnsafe");
    uf.setAccessible(true);
    sun.misc.Unsafe u = (sun.misc.Unsafe) uf.get(null);
    DecompilerContext ctx = (DecompilerContext) u.allocateInstance(DecompilerContext.class);
    setField(ctx, DecompilerContext.class, "counterContainer", new CounterContainer());
    setField(ctx, DecompilerContext.class, "properties", new HashMap<String, Object>(IFernflowerPreferences.getDefaults()));
    ClassesProcessor cp = (ClassesProcessor) u.allocateInstance(ClassesProcessor.class);
    setField(cp, ClassesProcessor.class, "mapRootClasses", new HashMap<>());
    setField(ctx, DecompilerContext.class, "classProcessor", cp);
    DecompilerContext.setCurrentContext(ctx);
    if (mt != null) {
      varproc = new VarProcessor(mt, mt.methodDescriptor());
    }
  }

  static VarExprent var(int index, String name, boolean def) {
    VarExprent var = new VarExprent(index, VarType.VARTYPE_INT, varproc);
    var.setDefinition(def);
    if (name != null) {
      varproc.setVarName(new VarVersionPair(index, 0), name);
    }
    return var;
  }

  // A block holding `var<index> = index`, the var a definition named `name`.
  static BasicBlockStatement assign(int index, String name) {
    BasicBlockStatement bb = BasicBlockStatement.create();
    bb.getExprents().add(new AssignmentExprent(var(index, name, true), new ConstExprent(index, true, null), null));
    return bb;
  }

  static SequenceStatement seq(Statement... stats) {
    SequenceStatement seq = new SequenceStatement(Arrays.asList(stats));
    seq.setAllParent();
    return seq;
  }

  static void run(String tag, Statement head) {
    RootStatement root = new RootStatement(head, new DummyExitStatement(), mt);
    VarDefinitionHelper helper = new VarDefinitionHelper(root, mt, varproc, false);
    helper.remapClashingNames(root, mt);
    StringBuilder sb = new StringBuilder(tag);
    for (Map.Entry<VarVersionPair, String> e : helper.getClashingNames().entrySet()) {
      sb.append(' ').append(e.getKey().var).append(',').append(e.getKey().version).append('=').append(e.getValue());
    }
    System.out.println(sb);
  }

  public static void main(String[] args) throws Exception {
    byte[] bytes = Files.readAllBytes(Path.of(args[0]));
    install();
    StructClass cl = StructClass.create(new DataInputFullStream(bytes), true);
    mt = cl.getMethod("straight", "(II)I");

    install();
    run("seq_clash", seq(assign(5, "a"), assign(6, "a")));

    install();
    run("nested_scope", seq(seq(assign(5, "a")), assign(6, "a")));

    install();
    run("triple", seq(assign(5, "a"), assign(6, "a"), assign(7, "a")));

    install();
    varproc.setVarName(new VarVersionPair(1, 0), "x");
    run("param_clash", seq(assign(5, "x"), assign(6, "x")));

    install();
    SequenceStatement withDefs = seq(assign(8, "b"));
    withDefs.getVarDefinitions().add(var(7, "b", true));
    run("var_definitions", withDefs);

    install();
    BasicBlockStatement bare = BasicBlockStatement.create();
    bare.getExprents().add(var(9, "c", true));
    bare.getExprents().add(var(9, "c", false));
    BasicBlockStatement reuse = BasicBlockStatement.create();
    reuse.getExprents().add(new AssignmentExprent(var(10, "c", true), var(9, null, false), null));
    run("bare_def", seq(bare, reuse, assign(11, "cx"), assign(12, "unnamed_ok")));

    install();
    List<Statement> many = new ArrayList<>();
    for (int i = 0; i < 20; i++) {
      many.add(assign(20 + i * 7, "v"));
    }
    run("many", seq(many.toArray(new Statement[0])));
  }
}
