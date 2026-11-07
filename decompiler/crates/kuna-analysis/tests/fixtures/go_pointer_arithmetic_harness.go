package main
import ("fmt"; "runtime"; "unsafe")
/* EMITTED_BODIES */
func main() {
    var observations int
    for seed:=uint64(0);seed<64;seed++ {
        var data [32]WORD
        var expected [32]WORD
        for i:=range data { data[i]=WORD((seed+1)*0x123456789abcdef ^ (uint64(i)+3)*0xfedcba987654321);expected[i]=data[i] }
        for center:=1;center<31;center++ {
            base:=unsafe.Pointer(&data[center])
            if load_offset(base)!=expected[center+1] || return_offset(base)!=&data[center+1] || return_negative(base)!=&data[center-1] {panic("constant offset")}
            observations+=3
            for index:=0;index<32;index++ {
                offset:=SIGNED(index-center)*SIGNED(unsafe.Sizeof(data[0]))
                if load_dynamic(base,offset)!=expected[index] || return_dynamic(base,offset)!=&data[index] {panic("dynamic offset")}
                value:=WORD(seed^uint64(index)) ^ ^WORD(0)
                store_dynamic(base,offset,value);expected[index]=value
                if data!=expected {panic("dynamic store canary")}
                observations+=3
            }
            store_offset(base,WORD(seed));expected[center+1]=WORD(seed)
            if data!=expected {panic("constant store canary")};observations++
            pointer:=return_offset(base)
            if center==16 {runtime.GC()}
            if *pointer!=expected[center+1] {panic("pointer lifetime")};observations++
        }
    }
    fmt.Printf("PASS observations=%d\n",observations)
}
