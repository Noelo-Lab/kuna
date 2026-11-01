func write_pair(ctx uint64) {
  var v1 *uint64 // rcx | stack - 0x8
  var v2 [16]byte // rbx:rax
  
  v1 = *(**uint64)(unsafe.Pointer(uintptr(ctx + 0x8)))
  v2 = pair(*(*uint64)(unsafe.Pointer(uintptr(ctx))))
  *v1 = SUB168(v2,0)
  *(*uint64)(unsafe.Add(unsafe.Pointer(v1),1 * 8)) = SUB168(v2,8)
  return
}
