from common import *
run(r,w,['cargo','build','--release','-p','kuna-cli','-p','kuna-console','-p','kuna-harness'],'build-final.log',r/'decompiler')
run(r,w,['cargo','test','-p','kuna-console','--test','integration','verify_gopairreturns::'],'console-focused.log',r/'decompiler')
