from common import *
import datetime,re
url=(w/'published.txt').read_text().strip();number=int(url.rsplit('/',1)[-1]);commit=json.loads((w/'commit.json').read_text());build=json.loads((w/'build.json').read_text());repo='Noelo-Lab/kuna'
def gh(*args):return subprocess.check_output(['gh',*args],cwd=r,text=True)
pr=json.loads(gh('pr','view',str(number),'--repo',repo,'--json','number,title,body,state,isDraft,baseRefName,headRefOid,mergeStateStatus,reviews,comments,statusCheckRollup'))
rest=json.loads(gh('api','repos/'+repo+'/pulls/'+str(number)))
pr['baseRefOid']=rest['base']['sha']
assert pr['headRefOid']==commit['head'] and pr['baseRefOid']==commit['parent_main']
assert pr['body']==(w/'pr-body.txt').read_text().rstrip('\n') or pr['body']==(w/'pr-body.txt').read_text()
assert pr['title']=='[AUTOMATED] Convert Rust array places at pointer consumers'
assert not subprocess.check_output(['git','status','--porcelain'],cwd=r)
assert hashlib.sha256((r/'decompiler/target/release/kuna').read_bytes()).hexdigest()==build['new_cli_sha256']
query='query($owner:String!,$name:String!,$number:Int!){repository(owner:$owner,name:$name){pullRequest(number:$number){reviewThreads(first:100){nodes{isResolved comments(first:100){nodes{author{login} body url}}}}}}}'
threads=json.loads(gh('api','graphql','-f','query='+query,'-f','owner=Noelo-Lab','-f','name=kuna','-F','number='+str(number)))
(w/'pr-fresh.json').write_text(json.dumps(pr,indent=2)+'\n');(w/'review-threads.json').write_text(json.dumps(threads,indent=2)+'\n')
print('PR',number,'merge',pr['mergeStateStatus'],'comments',len(pr['comments']),'reviews',len(pr['reviews']),'threads',len(threads['data']['repository']['pullRequest']['reviewThreads']['nodes']))
print([(check['name'],check.get('status'),check.get('conclusion'))for check in pr['statusCheckRollup']])
if pr['comments'] or pr['reviews'] or threads['data']['repository']['pullRequest']['reviewThreads']['nodes']:
 print('REVIEW CONTENT',json.dumps({'comments':pr['comments'],'reviews':pr['reviews'],'threads':threads},indent=2))
