from pathlib import Path
import angr,json,hashlib,subprocess,traceback,time
w=Path('/tmp/kuna-rust-array-zero-rebase');source=Path('/tmp/kuna-angr-reference/source');fixture=w/'probe';expected='6afa0c79a83d9f677a46b0275cdb63bf16c1979e';assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=source,text=True).strip()==expected
names=['zero_unsigned','zero_signed','zero_field','zero_second','zero_index','zero_callback','zero_bytes','one_unsigned','signed_input','unsigned_input','two_halves','copy_wide','zero_narrow'];p=angr.Project(str(fixture),auto_load_libs=False);cfg=p.analyses.CFGFast(normalize=True,show_progressbar=False);targets={name:p.kb.functions.function(name=name)for name in names};assert all(targets.values());p.analyses.CompleteCallingConventions(prioritize_func_addrs={f.addr for f in targets.values()},skip_other_funcs=True);rows=[]
for name,f in targets.items():
 start=time.monotonic();row={'name':name,'address':hex(f.addr)}
 try:
  dec=p.analyses.Decompiler(func=f,cfg=cfg.model,flavor='rust');assert dec.codegen is not None;row['code']=dec.codegen.text;assert row['code']
 except Exception as e:row['error']=type(e).__name__+': '+str(e);row['traceback']=traceback.format_exc()
 row['elapsed_seconds']=time.monotonic()-start;rows.append(row);(w/'angr-reference.json').write_text(json.dumps({'angr_commit':expected,'angr_version':angr.__version__,'fixture_sha256':hashlib.sha256(fixture.read_bytes()).hexdigest(),'functions':rows,'scope':'Complete pinned reference Rust text on the authored native fixture. No body edits or native-equivalence claim.'},indent=2)+'\n');print(name,'ERROR'if'error'in row else'CAPTURED',flush=True)
