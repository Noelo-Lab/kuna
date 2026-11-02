from common import *
run(r,w,["cargo","test","--release","-p","kuna-console","--test","integration","verify_rustprobestack::","--","--nocapture"],"console-focused-final.log",r/"decompiler")
run(r,w,["cargo","test","--release","-p","kuna-cli","--test","integration","rust_probestack::","--","--nocapture"],"cli-focused.log",r/"decompiler")
