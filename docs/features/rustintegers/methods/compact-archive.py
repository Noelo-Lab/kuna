from common import *
import gzip,lzma,shutil
out=r/'docs/features/rustintegers';old=out/'breadth/complete-changes.json.gz';new=out/'breadth/complete-changes.json.xz'
assert old.is_file() and not new.exists()
raw_hash=hashlib.sha256()
with gzip.open(old,'rb')as src,lzma.open(new,'wb',preset=6)as dst:
 while chunk:=src.read(1024*1024):raw_hash.update(chunk);dst.write(chunk)
assert raw_hash.hexdigest()==hashlib.sha256((w/'breadth/record.json').read_bytes()).hexdigest()
check=hashlib.sha256()
with lzma.open(new,'rb')as src:
 while chunk:=src.read(1024*1024):check.update(chunk)
assert check.hexdigest()==raw_hash.hexdigest()
assert new.stat().st_size<old.stat().st_size
old.unlink()
p=w/'finalize.py';s=p.read_text().replace('import datetime, gzip, re, shutil','import datetime, gzip, lzma, re, shutil');s=s.replace("compressed(w / 'breadth/record.json', 'breadth/complete-changes.json.gz')", "with (w / 'breadth/record.json').open('rb') as src, lzma.open(out / 'breadth/complete-changes.json.xz', 'wb', preset=6) as dst:\n    shutil.copyfileobj(src, dst)");p.write_text(s);shutil.copy2(p,out/'methods/finalize.py');shutil.copy2(w/'compact-archive.py',out/'methods/compact-archive.py')
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
manifest={str(p.relative_to(out)):sha(p)for p in sorted(out.rglob('*'))if p.is_file()and p.name!='manifest.json'}
(out/'manifest.json').write_text(json.dumps({'files':manifest},indent=2)+'\n')
assert all(sha(r/p)==digest for p,digest in json.loads((w/'production-source-before-validation.json').read_text()).items())
print('Lossless breadth archive',new.stat().st_size,'bytes; raw sha256',check.hexdigest(),flush=True)
