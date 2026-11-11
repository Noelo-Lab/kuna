package main
import "fmt"
type Pair struct { first int64; second int64 }
type UPair struct { first uint64; second uint64 }
var calls uint64
func make_pair(a,b int64) Pair { calls++; return Pair{a+b,a-b} }
func make_unsigned(a,b int64) UPair { calls++; return UPair{uint64(a)+uint64(b),uint64(a)-uint64(b)} }
/* EMITTED_CALLERS */
func main() { values:=[]int64{-9223372036854775808,-9223372036854775807,-4294967297,-4294967296,-65537,-65536,-32769,-32768,-257,-256,-129,-128,-1,0,1,127,128,255,256,32767,32768,65535,65536,4294967295,4294967296,4294967297,9223372036854775806,9223372036854775807}; for _,a:=range values { for _,b:=range values {
calls=0; fmt.Printf("%016x\n",uint64(signed_int8(a,b))); if calls!=1 {panic("signed_int8")}
calls=0; fmt.Printf("%016x\n",uint64(signed_int8_wide(a,b))); if calls!=1 {panic("signed_int8_wide")}
calls=0; fmt.Printf("%016x\n",uint64(signed_uint8(a,b))); if calls!=1 {panic("signed_uint8")}
calls=0; fmt.Printf("%016x\n",uint64(signed_uint8_wide(a,b))); if calls!=1 {panic("signed_uint8_wide")}
calls=0; fmt.Printf("%016x\n",uint64(signed_int16(a,b))); if calls!=1 {panic("signed_int16")}
calls=0; fmt.Printf("%016x\n",uint64(signed_int16_wide(a,b))); if calls!=1 {panic("signed_int16_wide")}
calls=0; fmt.Printf("%016x\n",uint64(signed_uint16(a,b))); if calls!=1 {panic("signed_uint16")}
calls=0; fmt.Printf("%016x\n",uint64(signed_uint16_wide(a,b))); if calls!=1 {panic("signed_uint16_wide")}
calls=0; fmt.Printf("%016x\n",uint64(signed_int32(a,b))); if calls!=1 {panic("signed_int32")}
calls=0; fmt.Printf("%016x\n",uint64(signed_int32_wide(a,b))); if calls!=1 {panic("signed_int32_wide")}
calls=0; fmt.Printf("%016x\n",uint64(signed_uint32(a,b))); if calls!=1 {panic("signed_uint32")}
calls=0; fmt.Printf("%016x\n",uint64(signed_uint32_wide(a,b))); if calls!=1 {panic("signed_uint32_wide")}
calls=0; fmt.Printf("%016x\n",uint64(unsigned_int8(a,b))); if calls!=1 {panic("unsigned_int8")}
calls=0; fmt.Printf("%016x\n",uint64(unsigned_uint8(a,b))); if calls!=1 {panic("unsigned_uint8")}
calls=0; fmt.Printf("%016x\n",uint64(unsigned_int16(a,b))); if calls!=1 {panic("unsigned_int16")}
calls=0; fmt.Printf("%016x\n",uint64(unsigned_uint16(a,b))); if calls!=1 {panic("unsigned_uint16")}
calls=0; fmt.Printf("%016x\n",uint64(unsigned_int32(a,b))); if calls!=1 {panic("unsigned_int32")}
calls=0; fmt.Printf("%016x\n",uint64(unsigned_uint32(a,b))); if calls!=1 {panic("unsigned_uint32")}
calls=0; fmt.Printf("%016x\n",uint64(first_narrow(a,b))); if calls!=1 {panic("first_narrow")}
calls=0; fmt.Printf("%016x\n",uint64(shift8(a,b))); if calls!=1 {panic("shift8")}
calls=0; fmt.Printf("%016x\n",uint64(shift32(a,b))); if calls!=1 {panic("shift32")}
calls=0; fmt.Printf("%016x\n",uint64(signed_high(a,b))); if calls!=1 {panic("signed_high")}
calls=0; fmt.Printf("%016x\n",uint64(signed_as_unsigned(a,b))); if calls!=1 {panic("signed_as_unsigned")}
calls=0; fmt.Printf("%016x\n",uint64(unsigned_as_signed(a,b))); if calls!=1 {panic("unsigned_as_signed")}
calls=0; fmt.Printf("%016x\n",uint64(exact_signed(a,b))); if calls!=1 {panic("exact_signed")}
calls=0; fmt.Printf("%016x\n",uint64(exact_unsigned(a,b))); if calls!=1 {panic("exact_unsigned")}
}}}
