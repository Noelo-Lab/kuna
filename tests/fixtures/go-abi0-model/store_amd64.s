#include "textflag.h"

TEXT ·store(SB), NOSPLIT, $0-16
	MOVQ a+0(FP), AX
	IMULQ $3, AX
	ADDQ b+8(FP), AX
	MOVQ AX, ·sink(SB)
	RET
