from common import *
from xml.sax.saxutils import escape
import re
names=['load_i64_s','store_i32_s','address_u16_u','field_load','callback_store','logical_address','signed_address']
lines=['<decompilertest>','<!-- P9 Rust raw-pointer PTRADD regression from the Go/Rust parity audit.','     Default output uses wrapping byte offsets; nocastprinting preserves the','     diagnostic path with raw-pointer indexing. Native CLI tests execute the','     complete bodies and compare memory, addresses, and callback order. -->','<binaryimage arch="x86:LE:64:default:gcc">','<bytechunk space="ram" offset="0x100000" readonly="true">c3</bytechunk>','<symbol space="ram" offset="0x100000" name="harness_stub"/>','</binaryimage>','<script>']
commands=['clear architecture','load file decompiler/crates/kuna-analysis/tests/fixtures/pointer_index_x86_64','read symbols','option outlang rust','option ctypes off','option realtypes off']
for line in(w/'native/pointer_index.kuna').read_text().splitlines():
 if line.split()[1]in names or line.split()[1].startswith('counted_'):
  commands.append('map '+line+';')
for name in names:commands+=['load function '+name,'decompile','print C']
commands+=['option nocastprinting on','load function load_i64_s','decompile','print C']
lines+=['  <com>'+escape(command)+'</com>'for command in commands];lines+=['</script>']
checks=[('Rust indexed load dereferences a wrapping byte pointer','return *p.wrapping_byte_offset((i as isize).wrapping_mul(8));'),('Rust indexed store preserves the four-byte scale','*p.wrapping_byte_offset((i as isize).wrapping_mul(4)) = value;'),('Rust indexed address does not borrow its pointee','return p.wrapping_byte_offset((i as isize).wrapping_mul(2));'),('Rust indexed record field preserves its byte scale','return (*p.wrapping_byte_offset((i as isize).wrapping_mul(16))).value;'),('Rust indexed callback store evaluates the pointer once','*counted_base(p).wrapping_byte_offset((v2 as isize).wrapping_mul(8)) = v1;'),('Rust logical index keeps its unsigned shift width','::core::convert::identity::<u64>(0xffffffffffffffff >> (n as u8 & 0x3f))'),('Rust arithmetic index keeps its signed shift width','::core::convert::identity::<i64>(-0x8000000000000000 >> (n as u8 & 0x3f))'),('Rust pointer index cast suppression keeps diagnostic output','return p[i];')]
for name,code in checks:lines.append('<stringmatch name="'+name+'" min="1" max="1">'+escape(re.escape(code).replace('\\ ', ' '))+'</stringmatch>')
lines+=['</decompilertest>'];(r/'tests/stages/ghangr-rust-pointer-index.xml').write_text('\n'.join(lines)+'\n');print('Wrote',len(checks),'stage assertions')
