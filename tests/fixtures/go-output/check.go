package main
import("bytes";"fmt";"math";"unsafe")
func expect(ok bool, label string) {if !ok{panic(label)}}
func main(){
 for a:=int64(-16);a<24;a++{for b:=int64(-5);b<9;b++{
  expect(add(a,b)==a+b,"add")
  expect(call_callback(callback,a)==a*3+1,"function pointer call")
  wanted:=b*2;if a>10{wanted=b+1}else if a<0{wanted=b-1};expect(choose(a,b)==wanted,"choose")
  wanted=2;if a>0 && a+b>4{wanted=7};expect(short_calls(a,b)==wanted,"short circuit")
  wanted=0;for n:=a;n>0 && n+b>4;n--{wanted+=n};expect(loop_calls(a,b)==wanted,"loop condition")
  wanted=0;n:=a;for{n--;if n&1!=0{wanted+=n};if n<=0{break}};expect(bottom(a)==wanted,"bottom test")
  wanted=b;switch a{case 0:wanted+=3;fallthrough;case 1:wanted*=2;case 2,3:wanted-=4;case 4:wanted^=7;case 5:wanted+=8;default:wanted=-wanted};expect(dispatch(a,b)==wanted,"switch")
 }}
 for n:=int64(-2);n<100;n++{var sum int64;for i:=int64(0);i<n;i++{if i&1!=0{sum+=i}};expect(sum_for(n)==sum,"for continue")}
 for n:=int64(-2);n<9;n++{
  var got, want int64
  ret:=bottom_continue(n,&got);x:=n
  for {want=want*10+1;x--;if x&1!=0{want=want*10+2}else{want=want*10+3};if x<=0{break}}
  expect(ret==x && got==want,"bottom test side effects")
  got=0;v:=condition_calls(n,&got);want=0;if n>0{want=1};expected:=int64(3);if n>2{expected=7};expect(v==expected && got==want,"short circuit side effects")
  got=0;v=condition_loop(n,&got);want=0;expected=0;for x:=n;x>0;x--{want++;if x<=2{break};expected+=x};expect(v==expected && got==want,"loop side effects")
  got=0;v=fallthroughs(uint64(n),10,&got);want=0;expected=10;switch n{case 0:want=12;expected=11;case 1:want=2;expected=11;case 2:want=34;expected=12;case 3:want=4;expected=12;case 4:expected=9;case 5:expected=8};expect(v==expected && got==want,"fallthrough side effects")
 }
 for n:=int64(-2);n<15;n++{
  var got int64;v:=condition_value(n,&got);want:=int64(0);if n>0{want=1};expected:=int64(3);if n>2{expected=n*2};expect(v==expected && got==want,"short circuit retained value")
  got=0;v=condition_for(n,&got);want=0;expected=0;for i:=int64(0);;i++{want++;if n-i<=2{break};if i&1==0{expected+=n-i}};expect(v==expected && got==want,"for condition retained value")
  expected=0;for x:=n;;x--{expected+=x;if x<=1{break}};expect(name_collision(n)==expected,"loop temporary collision")
  expect(_kuna_ident_676f(n,3)==n+3,"keyword and escaped-prefix parameters")
 }
 for a:=uint64(0);a<100;a++{for b:=uint64(0);b<64;b++{c:=a*17+b;expect(tiers(a,b,c)==(a|b)^(c+(a&(b>>3))),"precedence")}}
 for _,raw:=range []uint64{0,1,0x8000000000000000,0x3ff0000000000000,0x7ff0000000000000,0xfff0000000000000,0x7ff8000000000123,0xffffffffffffffff}{
  expect(math.Float64bits(bits(raw))==raw,"float bits");expect(unbits(math.Float64frombits(raw))==raw,"float unbits");expect(narrow(raw)==uint32(raw),"narrow")
 }
 for i:=-128;i<128;i++{expect(extend(int8(i))==int64(int8(i)),"signed extend");expect(number(int64(i))==float64(i),"numeric conversion")}
 xs:=[4]int32{4,-7,0,100};for i:=range xs{expect(read_at(&xs[0],int64(i))==xs[i],"pointer read");expect(shadow(&xs[0],int64(i))==xs[i],"predeclared and import shadow")};write_at(&xs[0],2,-31);expect(xs[2]==-31,"pointer write")
 expect(from_word(to_word(&xs[0]))==&xs[0],"pointer conversions")
 f:=1.25;expect(unsafe.Pointer(retype(&f))==unsafe.Pointer(&f),"pointer retype")
 want:=[]byte("quote\" tab\t utf8é\x00");got:=unsafe.Slice((*byte)(unsafe.Pointer(text())),len(want));expect(bytes.Equal(got,want),"string contents and terminator")
 expect(_kuna_ident_72756e65()==0x1f600,"rune")
 expect(null_pointer()==nil && is_null(nil) && !is_null(&xs[0]),"nil constants")
 expect(shadow_null(nil,7)==7 && shadow_null(&xs[0],7)==0,"nil variable collision")
 expect(bool_true() && !bool_false(),"boolean constants")
 for n:=int64(-3);n<8;n++{expect(bool_value(n)==(n!=0),"integer boolean");expected:=int64(3);if n!=0{expected=7};expect(bool_number(n!=0)==expected,"boolean integer");var calls int64;v:=nested_calls(n,&calls);want:=int64(0);if n>0{want=1;if n<=2{want++}};expect(v==(n>2) && calls==want,"nested short circuit effects")}
 fmt.Println("Go output semantic checks passed")
}
