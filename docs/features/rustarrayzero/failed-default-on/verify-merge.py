from common import *
number=int((w/'published.txt').read_text().strip().rsplit('/',1)[-1]);commit=json.loads((w/'commit.json').read_text());build=json.loads((w/'build.json').read_text())
result=json.loads(subprocess.check_output(['gh','pr','view',str(number),'--repo','Noelo-Lab/kuna','--json','state,mergedAt,mergeCommit,headRefOid,url'],cwd=r,text=True));assert result['state']=='MERGED'and result['headRefOid']==commit['head'];actual=result['mergeCommit']['oid']
subprocess.run(['git','fetch','origin','main'],cwd=r,check=True)
tree=subprocess.check_output(['git','rev-parse',actual+'^{tree}'],cwd=r,text=True).strip();assert tree==commit['tree'];result.update(merged_commit=actual,merged_tree=tree,published_commit=commit['head'],published_tree=commit['tree'],candidate_cli_sha256=build['new_cli_sha256'])
(w/'merged.json').write_text(json.dumps(result,indent=2)+'\n');print('MERGED',result['url'],actual,tree,flush=True)
