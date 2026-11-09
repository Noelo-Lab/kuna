@Deprecated
module kuna.sample {
  requires java.logging;
  requires transitive java.sql;
  requires static java.compiler;
  exports p;
  exports q to java.base, java.logging;
  opens p;
  opens q to java.sql;
  uses java.lang.Runnable;
  provides java.lang.Runnable with p.Impl, q.Other;
}
