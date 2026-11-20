.syntax unified
.thumb
.text
.global thumb_probe
.type thumb_probe,%function
.thumb_func
thumb_probe:
 movs r0, #7
 adds r0, #1
 adds r0, #1
 adds r0, #1
 bx lr
.size thumb_probe, .-thumb_probe
