#!/usr/bin/env bash
# Compile Vineflower's testData/src/<set> source sets the way build.gradle's
# createJavaTestDataSet / compileTestDataJasm do, into <OUT>/<set>/.
#
# Usage: compile_sets.sh OUT_CLASSES_DIR LOG_DIR [set...]
# Env:   KUNA_JVM_ORACLE, or the overrides VF_SRC (vineflower checkout), JDKS (dir with jdk8 jdk9 jdk11 jdk16
#        jdk17 jdk21 jdk25 links), CORPUS (holds the built jasm wrapper + the asmtools jar)
set -euo pipefail

OUT=${1:?usage: compile_sets.sh OUT_CLASSES_DIR LOG_DIR [set...]}
LOGS=${2:?usage: compile_sets.sh OUT_CLASSES_DIR LOG_DIR [set...]}
shift 2
NO_ORACLE='set it to the JVM oracle workspace (decompiler/nonnative/kuna-jvm/tools/oracle/README.md)'
VF_SRC=${VF_SRC:-${KUNA_JVM_ORACLE:?$NO_ORACLE}/vineflower}
JDKS=${JDKS:-${KUNA_JVM_ORACLE:?$NO_ORACLE}/jdks}
CORPUS=${CORPUS:-${KUNA_JVM_ORACLE:?$NO_ORACLE}/corpus}
SRC=$VF_SRC/testData/src

# set -> "javaVersion|extra javac args" (build.gradle lines 132-139)
declare -A SETS=(
  [java8]="8|"
  [java9]="9|"
  [java11]="11|"
  [java16]="16|"
  [java17]="17|"
  [java21]="21|"
  [java25]="25|"
  [java16preview]="16|--enable-preview"
  [java17preview]="17|--enable-preview"
  [java21preview]="21|--enable-preview"
  [java8nodebug]="8|-g:none"
  [java16nodebug]="16|-g:none"
)
ORDER=(java8 java8nodebug java9 java11 java16 java16nodebug java16preview java17 java17preview java21 java21preview java25 jasm)
if [ $# -gt 0 ]; then ORDER=("$@"); fi

mkdir -p "$OUT" "$LOGS"

info() {  # set javaVersion jdkHome compilerVersion nSources dest args... -> $LOGS/compile-info-<set>.json
  local set=$1 ver=$2 home=$3 comp=$4 n=$5 dest=$6
  shift 6
  python3 -I -c 'import json,sys,os
s,ver,home,comp,n,dest,out=sys.argv[1:8]
classes=sum(len([f for f in fs if f.endswith(".class")]) for _,_,fs in os.walk(dest))
json.dump(dict(set=s, java_version=int(ver) if ver else None, jdk_home=home, compiler=comp, sources=int(n),
               classes=classes, args=sys.argv[8:]), open(out,"w"), indent=1)' \
    "$set" "$ver" "$home" "$comp" "$n" "$dest" "$LOGS/compile-info-$set.json" "$@"
}

for set in "${ORDER[@]}"; do
  dest=$OUT/$set
  mkdir -p "$dest"
  log=$LOGS/compile-$set.log
  if [ "$set" = jasm ]; then
    # buildSrc JasmCompile: new org.openjdk.asmtools.jasm.Main(PrintWriter, "jasm").compile(["-g", "-d", dest, *.jasm])
    mapfile -t files < <(find "$SRC/jasm" -type f -name '*.jasm' | LC_ALL=C sort)
    if ! "$JDKS/jdk17/bin/java" -cp "$CORPUS/driver/classes:$CORPUS/tools/asmtools/asmtools-core-7.0.b10-ea.jar" \
        corpus.JasmCompile -g -d "$dest" "${files[@]}" > "$log" 2>&1; then
      echo "jasm failed, see $log" >&2; exit 1
    fi
    info "$set" "" "$(readlink -f "$JDKS/jdk17")" "org.openjdk.asmtools:asmtools-core:7.0.b10-ea (jasm), run on $("$JDKS/jdk17/bin/java" -version 2>&1 | sed -n 2p)" \
      "${#files[@]}" "$dest" -g -d "$dest" "<sources>"
    echo "compiled $set: ${#files[@]} sources with asmtools-core 7.0.b10-ea on $("$JDKS/jdk17/bin/java" -version 2>&1 | sed -n 2p)"
    continue
  fi
  spec=${SETS[$set]:?unknown set $set}
  ver=${spec%%|*}
  extra=${spec#*|}
  javac=$JDKS/jdk$ver/bin/javac
  srcver=$ver
  [ "$ver" = 8 ] && srcver=1.8
  argfile=$LOGS/sources-$set.txt
  find "$SRC/$set" -type f -name '*.java' | LC_ALL=C sort > "$argfile"
  n=$(wc -l < "$argfile")
  # A source set containing module-info.java (only java9) is compiled as a module; Gradle then puts the
  # source roots on -sourcepath (javac 9 rejects a module compile with an empty -sourcepath:
  # "file should be on source path, or on patch path for module"). Otherwise Gradle passes -sourcepath "".
  sourcepath=""
  if grep -q '/module-info\.java$' "$argfile"; then sourcepath=$SRC/$set; fi
  # Gradle JavaCompilerArgumentsBuilder order: -source -target -d -encoding -g -sourcepath "" -proc:none
  # -XDuseUnsharedTable=true, then options.compilerArgs, then the source files.
  # shellcheck disable=SC2086
  if ! "$javac" -source "$srcver" -target "$srcver" -d "$dest" -encoding UTF-8 -g -sourcepath "$sourcepath" -proc:none \
      -XDuseUnsharedTable=true $extra @"$argfile" > "$log" 2>&1; then
    echo "javac failed for $set, see $log" >&2; exit 1
  fi
  # shellcheck disable=SC2086
  info "$set" "$ver" "$(readlink -f "$JDKS/jdk$ver")" "$("$javac" -version 2>&1)" "$n" "$dest" \
    -source "$srcver" -target "$srcver" -d "$dest" -encoding UTF-8 -g -sourcepath "$sourcepath" -proc:none \
    -XDuseUnsharedTable=true $extra "<sources>"
  echo "compiled $set: $n sources with $("$javac" -version 2>&1) ${extra:+($extra)}"
done
