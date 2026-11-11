package main
import("fmt";"math";"unsafe")
type Fields struct{s8 int8;u8 byte;s16 int16;u16 uint16;s32 int32;u32 uint32;s64 int64;u64 uint64}
var pointer_calls,value_calls uint32
func counted_destination(p *Fields)*Fields{pointer_calls++;return p}
func counted_value(a,b uint64)uint64{value_calls++;return a+b}
func signed8(p *Fields, a byte, b byte) {
  (*p).s8 = b + a
}
func unsigned8(p *Fields, a int8, b int8) {
  (*p).u8 = b + a
}
func signed_plain8(p *int8, a byte, b byte) {
  *p = b + a
}
func unsigned_plain8(p *byte, a int8, b int8) {
  *p = b + a
}
func signed_xor8(p *Fields, a int8) {
  (*p).s8 = byte(a) ^ 0x1
}
func signed_constant8(p *Fields) {
  (*p).s8 = -1
}
func unsigned_constant8(p *Fields) {
  (*p).u8 = 0xff
}
func signed16(p *Fields, a uint16, b uint16) {
  (*p).s16 = b + a
}
func unsigned16(p *Fields, a int16, b int16) {
  (*p).u16 = b + a
}
func signed_plain16(p *int16, a uint16, b uint16) {
  *p = b + a
}
func unsigned_plain16(p *uint16, a int16, b int16) {
  *p = b + a
}
func signed_xor16(p *Fields, a int16) {
  (*p).s16 = uint16(a) ^ 0x1
}
func signed_constant16(p *Fields) {
  (*p).s16 = -1
}
func unsigned_constant16(p *Fields) {
  (*p).u16 = 0xffff
}
func signed32(p *Fields, a uint32, b uint32) {
  (*p).s32 = a + b
}
func unsigned32(p *Fields, a int32, b int32) {
  (*p).u32 = a + b
}
func signed_plain32(p *int32, a uint32, b uint32) {
  *p = a + b
}
func unsigned_plain32(p *uint32, a int32, b int32) {
  *p = a + b
}
func signed_xor32(p *Fields, a int32) {
  (*p).s32 = uint32(a) ^ 0x1
}
func signed_constant32(p *Fields) {
  (*p).s32 = -1
}
func unsigned_constant32(p *Fields) {
  (*p).u32 = 0xffffffff
}
func signed64(p *Fields, a uint64, b uint64) {
  (*p).s64 = a + b
}
func unsigned64(p *Fields, a int64, b int64) {
  (*p).u64 = a + b
}
func signed_plain64(p *int64, a uint64, b uint64) {
  *p = a + b
}
func unsigned_plain64(p *uint64, a int64, b int64) {
  *p = a + b
}
func signed_xor64(p *Fields, a int64) {
  (*p).s64 = uint64(a) ^ 0x1
}
func signed_constant64(p *Fields) {
  (*p).s64 = -1
}
func unsigned_constant64(p *Fields) {
  (*p).u64 = 0xffffffffffffffff
}
func side_effect(p *Fields, a uint64, b uint64) {
  var v1 uint64 // rax | rbx
  
  v1 = counted_value(a,b)
  (*counted_destination(p)).s64 = v1
}
func array_store(p *int64, i uint64, a uint64) {
  *(*int64)(unsafe.Add(unsafe.Pointer(p),i * 8)) = a
}
func float_store(p *float64, a float64) {
  *p = a
}
func pointer_store(p **int64, a *int64) {
  *p = a
}
func bool_store(p *bool, a bool) {
  *p = a
}
func main(){
values:=[]uint64{0,1,2,127,128,255,256,32767,32768,65535,65536,2147483647,2147483648,4294967295,4294967296,9223372036854775807,9223372036854775808,18446744073709551615}
for _,a:=range values {for _,b:=range values {
{p:=Fields{-3,5,-7,11,-13,17,-19,23};pointer_calls=0;value_calls=0;signed8(&p,uint8(a),uint8(b));fmt.Println("signed8",uint64(p.s8),p.u8,uint64(p.s16),p.u16,uint64(p.s32),p.u32,uint64(p.s64),p.u64,pointer_calls,value_calls)}
{p:=Fields{-3,5,-7,11,-13,17,-19,23};pointer_calls=0;value_calls=0;unsigned8(&p,int8(a),int8(b));fmt.Println("unsigned8",uint64(p.s8),p.u8,uint64(p.s16),p.u16,uint64(p.s32),p.u32,uint64(p.s64),p.u64,pointer_calls,value_calls)}
{p:=Fields{-3,5,-7,11,-13,17,-19,23};pointer_calls=0;value_calls=0;signed_plain8(&p.s8,uint8(a),uint8(b));fmt.Println("signed_plain8",uint64(p.s8),p.u8,uint64(p.s16),p.u16,uint64(p.s32),p.u32,uint64(p.s64),p.u64,pointer_calls,value_calls)}
{p:=Fields{-3,5,-7,11,-13,17,-19,23};pointer_calls=0;value_calls=0;unsigned_plain8(&p.u8,int8(a),int8(b));fmt.Println("unsigned_plain8",uint64(p.s8),p.u8,uint64(p.s16),p.u16,uint64(p.s32),p.u32,uint64(p.s64),p.u64,pointer_calls,value_calls)}
{p:=Fields{-3,5,-7,11,-13,17,-19,23};pointer_calls=0;value_calls=0;signed_xor8(&p,int8(a));fmt.Println("signed_xor8",uint64(p.s8),p.u8,uint64(p.s16),p.u16,uint64(p.s32),p.u32,uint64(p.s64),p.u64,pointer_calls,value_calls)}
{p:=Fields{-3,5,-7,11,-13,17,-19,23};pointer_calls=0;value_calls=0;signed_constant8(&p);fmt.Println("signed_constant8",uint64(p.s8),p.u8,uint64(p.s16),p.u16,uint64(p.s32),p.u32,uint64(p.s64),p.u64,pointer_calls,value_calls)}
{p:=Fields{-3,5,-7,11,-13,17,-19,23};pointer_calls=0;value_calls=0;unsigned_constant8(&p);fmt.Println("unsigned_constant8",uint64(p.s8),p.u8,uint64(p.s16),p.u16,uint64(p.s32),p.u32,uint64(p.s64),p.u64,pointer_calls,value_calls)}
{p:=Fields{-3,5,-7,11,-13,17,-19,23};pointer_calls=0;value_calls=0;signed16(&p,uint16(a),uint16(b));fmt.Println("signed16",uint64(p.s8),p.u8,uint64(p.s16),p.u16,uint64(p.s32),p.u32,uint64(p.s64),p.u64,pointer_calls,value_calls)}
{p:=Fields{-3,5,-7,11,-13,17,-19,23};pointer_calls=0;value_calls=0;unsigned16(&p,int16(a),int16(b));fmt.Println("unsigned16",uint64(p.s8),p.u8,uint64(p.s16),p.u16,uint64(p.s32),p.u32,uint64(p.s64),p.u64,pointer_calls,value_calls)}
{p:=Fields{-3,5,-7,11,-13,17,-19,23};pointer_calls=0;value_calls=0;signed_plain16(&p.s16,uint16(a),uint16(b));fmt.Println("signed_plain16",uint64(p.s8),p.u8,uint64(p.s16),p.u16,uint64(p.s32),p.u32,uint64(p.s64),p.u64,pointer_calls,value_calls)}
{p:=Fields{-3,5,-7,11,-13,17,-19,23};pointer_calls=0;value_calls=0;unsigned_plain16(&p.u16,int16(a),int16(b));fmt.Println("unsigned_plain16",uint64(p.s8),p.u8,uint64(p.s16),p.u16,uint64(p.s32),p.u32,uint64(p.s64),p.u64,pointer_calls,value_calls)}
{p:=Fields{-3,5,-7,11,-13,17,-19,23};pointer_calls=0;value_calls=0;signed_xor16(&p,int16(a));fmt.Println("signed_xor16",uint64(p.s8),p.u8,uint64(p.s16),p.u16,uint64(p.s32),p.u32,uint64(p.s64),p.u64,pointer_calls,value_calls)}
{p:=Fields{-3,5,-7,11,-13,17,-19,23};pointer_calls=0;value_calls=0;signed_constant16(&p);fmt.Println("signed_constant16",uint64(p.s8),p.u8,uint64(p.s16),p.u16,uint64(p.s32),p.u32,uint64(p.s64),p.u64,pointer_calls,value_calls)}
{p:=Fields{-3,5,-7,11,-13,17,-19,23};pointer_calls=0;value_calls=0;unsigned_constant16(&p);fmt.Println("unsigned_constant16",uint64(p.s8),p.u8,uint64(p.s16),p.u16,uint64(p.s32),p.u32,uint64(p.s64),p.u64,pointer_calls,value_calls)}
{p:=Fields{-3,5,-7,11,-13,17,-19,23};pointer_calls=0;value_calls=0;signed32(&p,uint32(a),uint32(b));fmt.Println("signed32",uint64(p.s8),p.u8,uint64(p.s16),p.u16,uint64(p.s32),p.u32,uint64(p.s64),p.u64,pointer_calls,value_calls)}
{p:=Fields{-3,5,-7,11,-13,17,-19,23};pointer_calls=0;value_calls=0;unsigned32(&p,int32(a),int32(b));fmt.Println("unsigned32",uint64(p.s8),p.u8,uint64(p.s16),p.u16,uint64(p.s32),p.u32,uint64(p.s64),p.u64,pointer_calls,value_calls)}
{p:=Fields{-3,5,-7,11,-13,17,-19,23};pointer_calls=0;value_calls=0;signed_plain32(&p.s32,uint32(a),uint32(b));fmt.Println("signed_plain32",uint64(p.s8),p.u8,uint64(p.s16),p.u16,uint64(p.s32),p.u32,uint64(p.s64),p.u64,pointer_calls,value_calls)}
{p:=Fields{-3,5,-7,11,-13,17,-19,23};pointer_calls=0;value_calls=0;unsigned_plain32(&p.u32,int32(a),int32(b));fmt.Println("unsigned_plain32",uint64(p.s8),p.u8,uint64(p.s16),p.u16,uint64(p.s32),p.u32,uint64(p.s64),p.u64,pointer_calls,value_calls)}
{p:=Fields{-3,5,-7,11,-13,17,-19,23};pointer_calls=0;value_calls=0;signed_xor32(&p,int32(a));fmt.Println("signed_xor32",uint64(p.s8),p.u8,uint64(p.s16),p.u16,uint64(p.s32),p.u32,uint64(p.s64),p.u64,pointer_calls,value_calls)}
{p:=Fields{-3,5,-7,11,-13,17,-19,23};pointer_calls=0;value_calls=0;signed_constant32(&p);fmt.Println("signed_constant32",uint64(p.s8),p.u8,uint64(p.s16),p.u16,uint64(p.s32),p.u32,uint64(p.s64),p.u64,pointer_calls,value_calls)}
{p:=Fields{-3,5,-7,11,-13,17,-19,23};pointer_calls=0;value_calls=0;unsigned_constant32(&p);fmt.Println("unsigned_constant32",uint64(p.s8),p.u8,uint64(p.s16),p.u16,uint64(p.s32),p.u32,uint64(p.s64),p.u64,pointer_calls,value_calls)}
{p:=Fields{-3,5,-7,11,-13,17,-19,23};pointer_calls=0;value_calls=0;signed64(&p,uint64(a),uint64(b));fmt.Println("signed64",uint64(p.s8),p.u8,uint64(p.s16),p.u16,uint64(p.s32),p.u32,uint64(p.s64),p.u64,pointer_calls,value_calls)}
{p:=Fields{-3,5,-7,11,-13,17,-19,23};pointer_calls=0;value_calls=0;unsigned64(&p,int64(a),int64(b));fmt.Println("unsigned64",uint64(p.s8),p.u8,uint64(p.s16),p.u16,uint64(p.s32),p.u32,uint64(p.s64),p.u64,pointer_calls,value_calls)}
{p:=Fields{-3,5,-7,11,-13,17,-19,23};pointer_calls=0;value_calls=0;signed_plain64(&p.s64,uint64(a),uint64(b));fmt.Println("signed_plain64",uint64(p.s8),p.u8,uint64(p.s16),p.u16,uint64(p.s32),p.u32,uint64(p.s64),p.u64,pointer_calls,value_calls)}
{p:=Fields{-3,5,-7,11,-13,17,-19,23};pointer_calls=0;value_calls=0;unsigned_plain64(&p.u64,int64(a),int64(b));fmt.Println("unsigned_plain64",uint64(p.s8),p.u8,uint64(p.s16),p.u16,uint64(p.s32),p.u32,uint64(p.s64),p.u64,pointer_calls,value_calls)}
{p:=Fields{-3,5,-7,11,-13,17,-19,23};pointer_calls=0;value_calls=0;signed_xor64(&p,int64(a));fmt.Println("signed_xor64",uint64(p.s8),p.u8,uint64(p.s16),p.u16,uint64(p.s32),p.u32,uint64(p.s64),p.u64,pointer_calls,value_calls)}
{p:=Fields{-3,5,-7,11,-13,17,-19,23};pointer_calls=0;value_calls=0;signed_constant64(&p);fmt.Println("signed_constant64",uint64(p.s8),p.u8,uint64(p.s16),p.u16,uint64(p.s32),p.u32,uint64(p.s64),p.u64,pointer_calls,value_calls)}
{p:=Fields{-3,5,-7,11,-13,17,-19,23};pointer_calls=0;value_calls=0;unsigned_constant64(&p);fmt.Println("unsigned_constant64",uint64(p.s8),p.u8,uint64(p.s16),p.u16,uint64(p.s32),p.u32,uint64(p.s64),p.u64,pointer_calls,value_calls)}
{p:=Fields{-3,5,-7,11,-13,17,-19,23};pointer_calls=0;value_calls=0;side_effect(&p,uint64(a),uint64(b));fmt.Println("side_effect",uint64(p.s8),p.u8,uint64(p.s16),p.u16,uint64(p.s32),p.u32,uint64(p.s64),p.u64,pointer_calls,value_calls)}
{arr:=[3]int64{9,10,11};array_store(&arr[0],a%3,b);fmt.Println("array_store",uint64(arr[0]),uint64(arr[1]),uint64(arr[2]))}
{var dest float64;float_store(&dest,math.Float64frombits(a));fmt.Println("float_store",math.Float64bits(dest))}
{value:=int64(a);var dest *int64;pointer_store(&dest,&value);fmt.Println("pointer_store",uint64(*dest))}
{var dest bool;bool_store(&dest,a&1!=0);v:=0;if dest{v=1};fmt.Println("bool_store",v)}
}}}
