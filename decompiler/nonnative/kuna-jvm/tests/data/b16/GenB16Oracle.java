// Golden vectors for the B16 slice (IfPatternMatchProcessor.getLastExprentWhen and
// isStatementMatchThrow), computed by the real Vineflower 1.12.0 classes.
// Regenerate with `sh gen.sh <vineflower-1.12.0.jar>`. Every scenario resets the
// counters to 1 and builds its exprents in the order tests/b16_pattern_match.rs mirrors.

import java.lang.reflect.Field;
import java.util.*;

import org.jetbrains.java.decompiler.main.ClassesProcessor;
import org.jetbrains.java.decompiler.main.DecompilerContext;
import org.jetbrains.java.decompiler.main.collectors.CounterContainer;
import org.jetbrains.java.decompiler.main.extern.IFernflowerPreferences;
import org.jetbrains.java.decompiler.modules.decompiler.IfPatternMatchProcessor;
import org.jetbrains.java.decompiler.modules.decompiler.exps.*;
import org.jetbrains.java.decompiler.modules.decompiler.exps.FunctionExprent.FunctionType;
import org.jetbrains.java.decompiler.modules.decompiler.stats.BasicBlockStatement;
import org.jetbrains.java.decompiler.struct.gen.CodeType;
import org.jetbrains.java.decompiler.struct.gen.VarType;

public class GenB16Oracle {
  static final StringBuilder OUT = new StringBuilder();

  static void p(Object... fields) {
    StringJoiner j = new StringJoiner(" ");
    for (Object f : fields) j.add(String.valueOf(f));
    OUT.append(j).append('\n');
  }

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
  }

  static void reset() {
    DecompilerContext.getCounterContainer().setCounter(CounterContainer.EXPRESSION_COUNTER, 1);
    DecompilerContext.getCounterContainer().setCounter(CounterContainer.STATEMENT_COUNTER, 1);
  }

  static VarType obj(String cls) {
    return new VarType(CodeType.OBJECT, 0, cls);
  }

  static VarExprent var(int index) {
    return new VarExprent(index, obj("java/lang/Object"), null);
  }

  static FunctionExprent fn(FunctionType t, Exprent... ops) {
    return new FunctionExprent(t, new ArrayList<>(Arrays.asList(ops)), null);
  }

  static FunctionExprent iof(int index, String cls) {
    return fn(FunctionType.INSTANCEOF, var(index), new ConstExprent(obj(cls), null, null));
  }

  static ConstExprent cb(int v) {
    return new ConstExprent(v, true, null);
  }

  static ConstExprent ci(int v) {
    return new ConstExprent(v, false, null);
  }

  static void last(String name, Exprent cond) {
    for (int t = 1; t >= 0; t--) {
      for (int o = 1; o >= 0; o--) {
        Exprent e = IfPatternMatchProcessor.getLastExprentWhen(cond, t == 1, o == 1);
        p("last", name, t, o, e == null ? "null" : e.id);
      }
    }
  }

  static ExitExprent exit(ExitExprent.Type t, Exprent value) {
    return new ExitExprent(t, value, null, null, null);
  }

  static NewExprent newObj(String cls) {
    return new NewExprent(obj(cls), new ArrayList<>(), null);
  }

  static void thr(String name, Exprent... exprents) {
    BasicBlockStatement bb = BasicBlockStatement.create();
    bb.getExprents().addAll(Arrays.asList(exprents));
    p("throw", name, IfPatternMatchProcessor.isStatementMatchThrow(bb) ? 1 : 0);
  }

  public static void main(String[] args) throws Exception {
    install();

    reset(); last("iof", iof(1, "java/lang/String"));
    reset(); last("var", var(1));
    reset(); last("and", fn(FunctionType.BOOLEAN_AND, iof(1, "A"), iof(2, "B")));
    reset(); last("or", fn(FunctionType.BOOLEAN_OR, iof(1, "A"), iof(2, "B")));
    reset(); last("not", fn(FunctionType.BOOL_NOT, iof(1, "A")));
    reset(); last("eq_false", fn(FunctionType.EQ, iof(1, "A"), cb(0)));
    reset(); last("eq_true", fn(FunctionType.EQ, iof(1, "A"), cb(1)));
    reset(); last("ne_false", fn(FunctionType.NE, iof(1, "A"), cb(0)));
    reset(); last("ne_true", fn(FunctionType.NE, iof(1, "A"), cb(1)));
    reset(); last("eq_int", fn(FunctionType.EQ, var(1), ci(0)));
    reset(); last("eq_boolcopy", fn(FunctionType.EQ, iof(1, "A"), new ConstExprent(new VarType(CodeType.BOOLEAN), 0, null)));
    reset(); last("ne_var", fn(FunctionType.NE, iof(1, "A"), var(2)));
    reset(); last("lt", fn(FunctionType.LT, var(1), var(2)));
    reset(); last("not_and", fn(FunctionType.BOOL_NOT, fn(FunctionType.BOOLEAN_AND, iof(1, "A"), iof(2, "B"))));
    reset(); last("nested", fn(FunctionType.BOOLEAN_AND,
      fn(FunctionType.BOOL_NOT, fn(FunctionType.BOOLEAN_OR, iof(1, "A"), iof(2, "B"))),
      fn(FunctionType.NE, iof(3, "C"), cb(0))));
    reset(); last("or_and", fn(FunctionType.BOOLEAN_OR, iof(1, "A"),
      fn(FunctionType.BOOLEAN_AND, iof(2, "B"), fn(FunctionType.EQ, iof(3, "C"), cb(1)))));

    reset(); thr("match", exit(ExitExprent.Type.THROW, newObj("java/lang/MatchException")));
    reset(); thr("other", exit(ExitExprent.Type.THROW, newObj("java/lang/IllegalStateException")));
    reset(); thr("two", exit(ExitExprent.Type.THROW, newObj("java/lang/MatchException")), var(1));
    reset(); thr("return", exit(ExitExprent.Type.RETURN, newObj("java/lang/MatchException")));
    reset(); thr("var", exit(ExitExprent.Type.THROW, var(1)));
    reset(); thr("not_exit", var(1));
    reset(); thr("empty");

    System.out.print(OUT);
  }
}
