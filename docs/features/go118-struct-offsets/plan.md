# Plan

1. Reproduce the opaque application structs on a stripped Go 1.18 executable.
2. Decode the old packed field word only when build-info identifies that era;
   preserve all existing name, extent and overlap checks.
3. Cover both layouts, embedded fields, malformed extents and unknown versions
   with factory-import tests and stripped amd64/arm64 builds.
4. Compare whole type surfaces and emitted C on old and new toolchains, measure
   load/decompile cost, and run the repository checks.
