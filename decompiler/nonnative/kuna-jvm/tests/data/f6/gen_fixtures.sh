#!/bin/sh
# Regenerates the F6 fixtures in this directory (needs JDK 21 javac/jar/jmod and python3).
# Afterwards run gen_golden.sh to refresh the Vineflower goldens.
set -eu
here=$(cd "$(dirname "$0")" && pwd)
work=$(mktemp -d)
javac --release 8 -d "$work/base" "$here"/src/base/pkg/Hello.java "$here"/src/base/pkg/util/Util.java
javac --release 11 -d "$work/v11" "$here"/src/v11/pkg/util/Util.java
javac --release 11 -d "$work/tiny" "$here"/src/tinymod/module-info.java "$here"/src/tinymod/f6/tiny/Tiny.java
python3 -I "$here/make_zips.py" "$work" "$here"
jar --create --file "$work/mrtool.jar" -C "$work/base" . --release 11 -C "$work/v11" .
cp "$work/mrtool.jar" "$here/mrtool.jar"
jmod create --module-version 1.0 --class-path "$work/tiny" "$work/tiny.jmod"
cp "$work/tiny.jmod" "$here/tiny.jmod"
mkdir -p "$here/tree/pkg/util" "$here/tree/libs" "$here/tree/res"
cp "$work/base/pkg/Hello.class" "$work/base/pkg/Hello\$1.class" "$here/tree/pkg/"
cp "$work/base/pkg/util/Util.class" "$here/tree/pkg/util/"
cp "$here/inner.jar" "$here/tree/libs/inner.jar"
cp "$here/prefixed.zip" "$here/tree/top.zip"
printf 'readme\n' > "$here/tree/readme.txt"
printf 'keep\n' > "$here/tree/res/.keep"
