"""Generate the LIBC_EXT additions from the reduced platform declarations.

Selection rule (mechanical): the name is an UNDEFINED FUNC symbol in >= 3 of the
665 dynamically linked binaries of the frozen decbench corpus, kuna's shipped
tables do not already carry it, and its platform declaration reduces whole to the
width-stable `Ty` vocabulary.
"""
import json,collections
ci=json.load(open('corpus-imports.json')); und=ci['und']
known=set(open('known-names.txt').read().split())
red=json.load(open('reduced.json'))
sel=[n for n in und if n not in known and n in red and und[n]>=3]
def unstable(n):
    r=red[n]
    return r['ret'].startswith('I64') or any(p.startswith('I64') for p in r['params'])
rejected_i64=sorted([n for n in sel if unstable(n)], key=lambda n:-und[n])
sel=[n for n in sel if not unstable(n)]
HDR={'stdio.h':'stdio.h','stdio2.h':'stdio.h','stdlib.h':'stdlib.h','string.h':'string.h','strings.h':'string.h',
     'unistd.h':'unistd.h','unistd_ext.h':'unistd.h','fcntl.h':'fcntl.h / sys/stat.h / dirent.h','fcntl2.h':'fcntl.h / sys/stat.h / dirent.h',
     'fcntl-linux.h':'fcntl.h / sys/stat.h / dirent.h','stat.h':'fcntl.h / sys/stat.h / dirent.h','dirent.h':'fcntl.h / sys/stat.h / dirent.h',
     'statx-generic.h':'fcntl.h / sys/stat.h / dirent.h','statvfs.h':'fcntl.h / sys/stat.h / dirent.h',
     'signal.h':'signal.h / sys/wait.h / setjmp.h','wait.h':'signal.h / sys/wait.h / setjmp.h','setjmp.h':'signal.h / sys/wait.h / setjmp.h',
     'setjmp2.h':'signal.h / sys/wait.h / setjmp.h','sigthread.h':'signal.h / sys/wait.h / setjmp.h',
     'socket.h':'sys/socket.h / netdb.h / arpa/inet.h','netdb.h':'sys/socket.h / netdb.h / arpa/inet.h','inet.h':'sys/socket.h / netdb.h / arpa/inet.h',
     'uio.h':'sys/socket.h / netdb.h / arpa/inet.h','select.h':'sys/socket.h / netdb.h / arpa/inet.h','select2.h':'sys/socket.h / netdb.h / arpa/inet.h',
     'ctype.h':'ctype.h / wctype.h / wchar.h','wctype-wchar.h':'ctype.h / wctype.h / wchar.h','wchar.h':'ctype.h / wctype.h / wchar.h','wchar2.h':'ctype.h / wctype.h / wchar.h',
     'time.h':'time.h / locale.h / libintl.h','libintl.h':'time.h / locale.h / libintl.h','utime.h':'time.h / locale.h / libintl.h',
     'grp.h':'pwd.h / grp.h / shadow.h / crypt.h / utmp.h','pwd.h':'pwd.h / grp.h / shadow.h / crypt.h / utmp.h',
     'shadow.h':'pwd.h / grp.h / shadow.h / crypt.h / utmp.h','crypt.h':'pwd.h / grp.h / shadow.h / crypt.h / utmp.h',
     'utmp.h':'pwd.h / grp.h / shadow.h / crypt.h / utmp.h','utmpx.h':'pwd.h / grp.h / shadow.h / crypt.h / utmp.h',
     'selinux.h':'selinux/selinux.h','pthread.h':'pthread.h / sched.h','sched.h':'pthread.h / sched.h','cpu-set.h':'pthread.h / sched.h',
     'xattr.h':'sys/xattr.h / sys/mman.h / mntent.h','mman.h':'sys/xattr.h / sys/mman.h / mntent.h','mntent.h':'sys/xattr.h / sys/mman.h / mntent.h',
     'glob.h':'the rest','termios.h':'the rest','libgen.h':'the rest','dlfcn.h':'the rest','file.h':'the rest','sysinfo.h':'the rest',
     'utsname.h':'the rest','error.h':'the rest','obstack.h':'the rest','getopt_core.h':'the rest'}
ORDER=['stdio.h','string.h','stdlib.h','ctype.h / wctype.h / wchar.h','time.h / locale.h / libintl.h','unistd.h',
       'fcntl.h / sys/stat.h / dirent.h','signal.h / sys/wait.h / setjmp.h','sys/socket.h / netdb.h / arpa/inet.h',
       'pwd.h / grp.h / shadow.h / crypt.h / utmp.h','selinux/selinux.h','pthread.h / sched.h',
       'sys/xattr.h / sys/mman.h / mntent.h','the rest']
groups=collections.defaultdict(list)
for n in sel:
    h=red[n]['src'].split('/')[-1].split(':')[0]
    groups[HDR.get(h,'the rest')].append(n)
lines=[]
for g in ORDER:
    if not groups.get(g): continue
    lines.append(f"    // {g}")
    for n in sorted(groups[g]):
        r=red[n]
        ps=', '.join(f'Ty::{p}' for p in r['params'])
        lines.append(f"    (\"{n}\", Sig {{ ret: Ty::{r['ret']}, params: &[{ps}], vararg: {r['vararg']} }}),")
leftover=set(sel)-{n for g in ORDER for n in groups.get(g,[])}
assert not leftover, leftover
open('table.rs.frag','w').write('\n'.join(lines)+'\n')
print('entries',len(sel))
print('rejected for a non-width-stable 64-bit slot:',len(rejected_i64), rejected_i64)
json.dump({'selected':sorted(sel),'rejected_i64':rejected_i64,'counts':{n:und[n] for n in sel}},open('selected.json','w'),indent=1)
