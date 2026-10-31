/* Float stores through a pointer whose other stores are integer bit patterns
   (clang lowers each 1.0f constant to an integer store of 0x3f800000), so kuna
   types the pointee as unknown bytes. See tests/stages/kuna-floatstore.xml. */
void mat_param(float *m, float a, float b) {
  m[0] = 1.0f;
  m[4] = 1.0f;
  m[2] = a * b + 0.5f;
}
