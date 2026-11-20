.set noreorder
.text
.globl bounce
.type bounce,@function
bounce:
 or $8,$31,$0
 andi $11,$4,1
 bnez $11,.Lsecond
 nop
 lui $10,%hi(.La0)
 ori $10,$10,%lo(.La0)
.Lprea:
 addiu $12,$12,1
.Ljumpa:
 jr $10
 nop
.La0:
 sw $4,0($5)
 addiu $4,$4,2
 lui $10,%hi(.La1)
 ori $10,$10,%lo(.La1)
 b .Lprea
 nop
.La1:
 sw $4,4($5)
 addiu $4,$4,2
 lui $10,%hi(.La2)
 ori $10,$10,%lo(.La2)
 b .Lprea
 nop
.La2:
 sw $4,8($5)
 addiu $4,$4,2
 lui $10,%hi(.La3)
 ori $10,$10,%lo(.La3)
 b .Lprea
 nop
.La3:
 sw $4,12($5)
 addiu $4,$4,2
 lui $10,%hi(.La4)
 ori $10,$10,%lo(.La4)
 b .Lprea
 nop
.La4:
 sw $4,16($5)
 addiu $4,$4,2
 lui $10,%hi(.La5)
 ori $10,$10,%lo(.La5)
 b .Lprea
 nop
.La5:
 sw $4,20($5)
 addiu $4,$4,2
 lui $10,%hi(.La6)
 ori $10,$10,%lo(.La6)
 b .Lprea
 nop
.La6:
 sw $4,24($5)
 addiu $4,$4,2
 lui $10,%hi(.La7)
 ori $10,$10,%lo(.La7)
 b .Lprea
 nop
.La7:
 sw $4,28($5)
 addiu $4,$4,2
 lui $10,%hi(.La8)
 ori $10,$10,%lo(.La8)
 b .Lprea
 nop
.La8:
 sw $4,32($5)
 addiu $4,$4,2
 lui $10,%hi(.La9)
 ori $10,$10,%lo(.La9)
 b .Lprea
 nop
.La9:
 sw $4,36($5)
 addiu $4,$4,2
 lui $10,%hi(.La10)
 ori $10,$10,%lo(.La10)
 b .Lprea
 nop
.La10:
 sw $4,40($5)
 addiu $4,$4,2
 lui $10,%hi(.La11)
 ori $10,$10,%lo(.La11)
 b .Lprea
 nop
.La11:
 sw $4,44($5)
 addiu $4,$4,2
 lui $10,%hi(.La12)
 ori $10,$10,%lo(.La12)
 b .Lprea
 nop
.La12:
 sw $4,48($5)
 addiu $4,$4,2
 lui $10,%hi(.La13)
 ori $10,$10,%lo(.La13)
 b .Lprea
 nop
.La13:
 sw $4,52($5)
 addiu $4,$4,2
 lui $10,%hi(.La14)
 ori $10,$10,%lo(.La14)
 b .Lprea
 nop
.La14:
 sw $4,56($5)
 addiu $4,$4,2
 lui $10,%hi(.La15)
 ori $10,$10,%lo(.La15)
 b .Lprea
 nop
.La15:
 sw $4,60($5)
 addiu $4,$4,2
 lui $10,%hi(.La16)
 ori $10,$10,%lo(.La16)
 b .Lprea
 nop
.La16:
 sw $4,64($5)
 addiu $4,$4,2
 lui $10,%hi(.La17)
 ori $10,$10,%lo(.La17)
 b .Lprea
 nop
.La17:
 sw $4,68($5)
 addiu $4,$4,2
 lui $10,%hi(.La18)
 ori $10,$10,%lo(.La18)
 b .Lprea
 nop
.La18:
 sw $4,72($5)
 addiu $4,$4,2
 lui $10,%hi(.La19)
 ori $10,$10,%lo(.La19)
 b .Lprea
 nop
.La19:
 sw $4,76($5)
 addiu $4,$4,2
 lui $10,%hi(.La20)
 ori $10,$10,%lo(.La20)
 b .Lprea
 nop
.La20:
 sw $4,80($5)
 addiu $4,$4,2
 lui $10,%hi(.La21)
 ori $10,$10,%lo(.La21)
 b .Lprea
 nop
.La21:
 sw $4,84($5)
 addiu $4,$4,2
 lui $10,%hi(.La22)
 ori $10,$10,%lo(.La22)
 b .Lprea
 nop
.La22:
 sw $4,88($5)
 addiu $4,$4,2
 lui $10,%hi(.La23)
 ori $10,$10,%lo(.La23)
 b .Lprea
 nop
.La23:
 sw $4,92($5)
 addiu $4,$4,2
 lui $10,%hi(.La24)
 ori $10,$10,%lo(.La24)
 b .Lprea
 nop
.La24:
 sw $4,96($5)
 addiu $4,$4,2
 lui $10,%hi(.La25)
 ori $10,$10,%lo(.La25)
 b .Lprea
 nop
.La25:
 sw $4,100($5)
 addiu $4,$4,2
 lui $10,%hi(.La26)
 ori $10,$10,%lo(.La26)
 b .Lprea
 nop
.La26:
 sw $4,104($5)
 addiu $4,$4,2
 lui $10,%hi(.La27)
 ori $10,$10,%lo(.La27)
 b .Lprea
 nop
.La27:
 sw $4,108($5)
 addiu $4,$4,2
 lui $10,%hi(.La28)
 ori $10,$10,%lo(.La28)
 b .Lprea
 nop
.La28:
 sw $4,112($5)
 addiu $4,$4,2
 lui $10,%hi(.La29)
 ori $10,$10,%lo(.La29)
 b .Lprea
 nop
.La29:
 sw $4,116($5)
 addiu $4,$4,2
 lui $10,%hi(.La30)
 ori $10,$10,%lo(.La30)
 b .Lprea
 nop
.La30:
 sw $4,120($5)
 addiu $4,$4,2
 lui $10,%hi(.La31)
 ori $10,$10,%lo(.La31)
 b .Lprea
 nop
.La31:
 sw $4,124($5)
 addiu $4,$4,2
 lui $10,%hi(.La32)
 ori $10,$10,%lo(.La32)
 b .Lprea
 nop
.La32:
 sw $4,128($5)
 addiu $4,$4,2
 lui $10,%hi(.La33)
 ori $10,$10,%lo(.La33)
 b .Lprea
 nop
.La33:
 sw $4,132($5)
 addiu $4,$4,2
 lui $10,%hi(.La34)
 ori $10,$10,%lo(.La34)
 b .Lprea
 nop
.La34:
 sw $4,136($5)
 addiu $4,$4,2
 lui $10,%hi(.La35)
 ori $10,$10,%lo(.La35)
 b .Lprea
 nop
.La35:
 sw $4,140($5)
 addiu $4,$4,2
 lui $10,%hi(.La36)
 ori $10,$10,%lo(.La36)
 b .Lprea
 nop
.La36:
 sw $4,144($5)
 addiu $4,$4,2
 lui $10,%hi(.La37)
 ori $10,$10,%lo(.La37)
 b .Lprea
 nop
.La37:
 sw $4,148($5)
 addiu $4,$4,2
 lui $10,%hi(.La38)
 ori $10,$10,%lo(.La38)
 b .Lprea
 nop
.La38:
 sw $4,152($5)
 addiu $4,$4,2
 lui $10,%hi(.La39)
 ori $10,$10,%lo(.La39)
 b .Lprea
 nop
.La39:
 sw $4,156($5)
 addiu $4,$4,2
 lui $10,%hi(.La40)
 ori $10,$10,%lo(.La40)
 b .Lprea
 nop
.La40:
 sw $4,160($5)
 addiu $4,$4,2
 lui $10,%hi(.La41)
 ori $10,$10,%lo(.La41)
 b .Lprea
 nop
.La41:
 sw $4,164($5)
 addiu $4,$4,2
 lui $10,%hi(.La42)
 ori $10,$10,%lo(.La42)
 b .Lprea
 nop
.La42:
 sw $4,168($5)
 addiu $4,$4,2
 lui $10,%hi(.La43)
 ori $10,$10,%lo(.La43)
 b .Lprea
 nop
.La43:
 sw $4,172($5)
 addiu $4,$4,2
 lui $10,%hi(.La44)
 ori $10,$10,%lo(.La44)
 b .Lprea
 nop
.La44:
 sw $4,176($5)
 addiu $4,$4,2
 lui $10,%hi(.La45)
 ori $10,$10,%lo(.La45)
 b .Lprea
 nop
.La45:
 sw $4,180($5)
 addiu $4,$4,2
 lui $10,%hi(.La46)
 ori $10,$10,%lo(.La46)
 b .Lprea
 nop
.La46:
 sw $4,184($5)
 addiu $4,$4,2
 lui $10,%hi(.La47)
 ori $10,$10,%lo(.La47)
 b .Lprea
 nop
.La47:
 sw $4,188($5)
 addiu $4,$4,2
 lui $10,%hi(.La48)
 ori $10,$10,%lo(.La48)
 b .Lprea
 nop
.La48:
 sw $4,192($5)
 addiu $4,$4,2
 lui $10,%hi(.La49)
 ori $10,$10,%lo(.La49)
 b .Lprea
 nop
.La49:
 sw $4,196($5)
 addiu $4,$4,2
 lui $10,%hi(.La50)
 ori $10,$10,%lo(.La50)
 b .Lprea
 nop
.La50:
 sw $4,200($5)
 addiu $4,$4,2
 lui $10,%hi(.La51)
 ori $10,$10,%lo(.La51)
 b .Lprea
 nop
.La51:
 sw $4,204($5)
 addiu $4,$4,2
 lui $10,%hi(.La52)
 ori $10,$10,%lo(.La52)
 b .Lprea
 nop
.La52:
 sw $4,208($5)
 addiu $4,$4,2
 lui $10,%hi(.La53)
 ori $10,$10,%lo(.La53)
 b .Lprea
 nop
.La53:
 sw $4,212($5)
 addiu $4,$4,2
 lui $10,%hi(.La54)
 ori $10,$10,%lo(.La54)
 b .Lprea
 nop
.La54:
 sw $4,216($5)
 addiu $4,$4,2
 lui $10,%hi(.La55)
 ori $10,$10,%lo(.La55)
 b .Lprea
 nop
.La55:
 sw $4,220($5)
 addiu $4,$4,2
 lui $10,%hi(.La56)
 ori $10,$10,%lo(.La56)
 b .Lprea
 nop
.La56:
 sw $4,224($5)
 addiu $4,$4,2
 lui $10,%hi(.La57)
 ori $10,$10,%lo(.La57)
 b .Lprea
 nop
.La57:
 sw $4,228($5)
 addiu $4,$4,2
 lui $10,%hi(.La58)
 ori $10,$10,%lo(.La58)
 b .Lprea
 nop
.La58:
 sw $4,232($5)
 addiu $4,$4,2
 lui $10,%hi(.La59)
 ori $10,$10,%lo(.La59)
 b .Lprea
 nop
.La59:
 sw $4,236($5)
 addiu $4,$4,2
 lui $10,%hi(.La60)
 ori $10,$10,%lo(.La60)
 b .Lprea
 nop
.La60:
 sw $4,240($5)
 addiu $4,$4,2
 lui $10,%hi(.La61)
 ori $10,$10,%lo(.La61)
 b .Lprea
 nop
.La61:
 sw $4,244($5)
 addiu $4,$4,2
 lui $10,%hi(.La62)
 ori $10,$10,%lo(.La62)
 b .Lprea
 nop
.La62:
 sw $4,248($5)
 addiu $4,$4,2
 lui $10,%hi(.La63)
 ori $10,$10,%lo(.La63)
 b .Lprea
 nop
.La63:
 sw $4,252($5)
 addiu $4,$4,2
 lui $10,%hi(.La64)
 ori $10,$10,%lo(.La64)
 b .Lprea
 nop
.La64:
 sw $4,256($5)
 addiu $4,$4,2
 lui $10,%hi(.La65)
 ori $10,$10,%lo(.La65)
 b .Lprea
 nop
.La65:
 sw $4,260($5)
 addiu $4,$4,2
 lui $10,%hi(.La66)
 ori $10,$10,%lo(.La66)
 b .Lprea
 nop
.La66:
 sw $4,264($5)
 addiu $4,$4,2
 lui $10,%hi(.La67)
 ori $10,$10,%lo(.La67)
 b .Lprea
 nop
.La67:
 sw $4,268($5)
 addiu $4,$4,2
 lui $10,%hi(.La68)
 ori $10,$10,%lo(.La68)
 b .Lprea
 nop
.La68:
 sw $4,272($5)
 addiu $4,$4,2
 lui $10,%hi(.La69)
 ori $10,$10,%lo(.La69)
 b .Lprea
 nop
.La69:
 sw $4,276($5)
 addiu $4,$4,2
 lui $10,%hi(.La70)
 ori $10,$10,%lo(.La70)
 b .Lprea
 nop
.La70:
 sw $4,280($5)
 addiu $4,$4,2
 lui $10,%hi(.La71)
 ori $10,$10,%lo(.La71)
 b .Lprea
 nop
.La71:
 sw $4,284($5)
 addiu $4,$4,2
 lui $10,%hi(.La72)
 ori $10,$10,%lo(.La72)
 b .Lprea
 nop
.La72:
 sw $4,288($5)
 addiu $4,$4,2
 lui $10,%hi(.La73)
 ori $10,$10,%lo(.La73)
 b .Lprea
 nop
.La73:
 sw $4,292($5)
 addiu $4,$4,2
 lui $10,%hi(.La74)
 ori $10,$10,%lo(.La74)
 b .Lprea
 nop
.La74:
 sw $4,296($5)
 addiu $4,$4,2
 lui $10,%hi(.La75)
 ori $10,$10,%lo(.La75)
 b .Lprea
 nop
.La75:
 sw $4,300($5)
 addiu $4,$4,2
 lui $10,%hi(.La76)
 ori $10,$10,%lo(.La76)
 b .Lprea
 nop
.La76:
 sw $4,304($5)
 addiu $4,$4,2
 lui $10,%hi(.La77)
 ori $10,$10,%lo(.La77)
 b .Lprea
 nop
.La77:
 sw $4,308($5)
 addiu $4,$4,2
 lui $10,%hi(.La78)
 ori $10,$10,%lo(.La78)
 b .Lprea
 nop
.La78:
 sw $4,312($5)
 addiu $4,$4,2
 lui $10,%hi(.La79)
 ori $10,$10,%lo(.La79)
 b .Lprea
 nop
.La79:
 sw $4,316($5)
 addiu $4,$4,2
 or $31,$8,$0
 jr $31
 nop
.Lsecond:
 lui $9,%hi(.Lb0)
 ori $9,$9,%lo(.Lb0)
.Lpreb:
 addiu $12,$12,1
.Ljumpb:
 jr $9
 nop
.Lb0:
 sw $4,0($5)
 addiu $4,$4,2
 lui $9,%hi(.Lb1)
 ori $9,$9,%lo(.Lb1)
 b .Lpreb
 nop
.Lb1:
 sw $4,4($5)
 addiu $4,$4,2
 lui $9,%hi(.Lb2)
 ori $9,$9,%lo(.Lb2)
 b .Lpreb
 nop
.Lb2:
 sw $4,8($5)
 addiu $4,$4,2
 lui $9,%hi(.Lb3)
 ori $9,$9,%lo(.Lb3)
 b .Lpreb
 nop
.Lb3:
 sw $4,12($5)
 addiu $4,$4,2
 lui $9,%hi(.Lb4)
 ori $9,$9,%lo(.Lb4)
 b .Lpreb
 nop
.Lb4:
 sw $4,16($5)
 addiu $4,$4,2
 lui $9,%hi(.Lb5)
 ori $9,$9,%lo(.Lb5)
 b .Lpreb
 nop
.Lb5:
 sw $4,20($5)
 addiu $4,$4,2
 lui $9,%hi(.Lb6)
 ori $9,$9,%lo(.Lb6)
 b .Lpreb
 nop
.Lb6:
 sw $4,24($5)
 addiu $4,$4,2
 lui $9,%hi(.Lb7)
 ori $9,$9,%lo(.Lb7)
 b .Lpreb
 nop
.Lb7:
 sw $4,28($5)
 addiu $4,$4,2
 lui $9,%hi(.Lb8)
 ori $9,$9,%lo(.Lb8)
 b .Lpreb
 nop
.Lb8:
 sw $4,32($5)
 addiu $4,$4,2
 lui $9,%hi(.Lb9)
 ori $9,$9,%lo(.Lb9)
 b .Lpreb
 nop
.Lb9:
 sw $4,36($5)
 addiu $4,$4,2
 lui $9,%hi(.Lb10)
 ori $9,$9,%lo(.Lb10)
 b .Lpreb
 nop
.Lb10:
 sw $4,40($5)
 addiu $4,$4,2
 lui $9,%hi(.Lb11)
 ori $9,$9,%lo(.Lb11)
 b .Lpreb
 nop
.Lb11:
 sw $4,44($5)
 addiu $4,$4,2
 lui $9,%hi(.Lb12)
 ori $9,$9,%lo(.Lb12)
 b .Lpreb
 nop
.Lb12:
 sw $4,48($5)
 addiu $4,$4,2
 lui $9,%hi(.Lb13)
 ori $9,$9,%lo(.Lb13)
 b .Lpreb
 nop
.Lb13:
 sw $4,52($5)
 addiu $4,$4,2
 lui $9,%hi(.Lb14)
 ori $9,$9,%lo(.Lb14)
 b .Lpreb
 nop
.Lb14:
 sw $4,56($5)
 addiu $4,$4,2
 lui $9,%hi(.Lb15)
 ori $9,$9,%lo(.Lb15)
 b .Lpreb
 nop
.Lb15:
 sw $4,60($5)
 addiu $4,$4,2
 lui $9,%hi(.Lb16)
 ori $9,$9,%lo(.Lb16)
 b .Lpreb
 nop
.Lb16:
 sw $4,64($5)
 addiu $4,$4,2
 lui $9,%hi(.Lb17)
 ori $9,$9,%lo(.Lb17)
 b .Lpreb
 nop
.Lb17:
 sw $4,68($5)
 addiu $4,$4,2
 lui $9,%hi(.Lb18)
 ori $9,$9,%lo(.Lb18)
 b .Lpreb
 nop
.Lb18:
 sw $4,72($5)
 addiu $4,$4,2
 lui $9,%hi(.Lb19)
 ori $9,$9,%lo(.Lb19)
 b .Lpreb
 nop
.Lb19:
 sw $4,76($5)
 addiu $4,$4,2
 lui $9,%hi(.Lb20)
 ori $9,$9,%lo(.Lb20)
 b .Lpreb
 nop
.Lb20:
 sw $4,80($5)
 addiu $4,$4,2
 lui $9,%hi(.Lb21)
 ori $9,$9,%lo(.Lb21)
 b .Lpreb
 nop
.Lb21:
 sw $4,84($5)
 addiu $4,$4,2
 lui $9,%hi(.Lb22)
 ori $9,$9,%lo(.Lb22)
 b .Lpreb
 nop
.Lb22:
 sw $4,88($5)
 addiu $4,$4,2
 lui $9,%hi(.Lb23)
 ori $9,$9,%lo(.Lb23)
 b .Lpreb
 nop
.Lb23:
 sw $4,92($5)
 addiu $4,$4,2
 lui $9,%hi(.Lb24)
 ori $9,$9,%lo(.Lb24)
 b .Lpreb
 nop
.Lb24:
 sw $4,96($5)
 addiu $4,$4,2
 lui $9,%hi(.Lb25)
 ori $9,$9,%lo(.Lb25)
 b .Lpreb
 nop
.Lb25:
 sw $4,100($5)
 addiu $4,$4,2
 lui $9,%hi(.Lb26)
 ori $9,$9,%lo(.Lb26)
 b .Lpreb
 nop
.Lb26:
 sw $4,104($5)
 addiu $4,$4,2
 lui $9,%hi(.Lb27)
 ori $9,$9,%lo(.Lb27)
 b .Lpreb
 nop
.Lb27:
 sw $4,108($5)
 addiu $4,$4,2
 lui $9,%hi(.Lb28)
 ori $9,$9,%lo(.Lb28)
 b .Lpreb
 nop
.Lb28:
 sw $4,112($5)
 addiu $4,$4,2
 lui $9,%hi(.Lb29)
 ori $9,$9,%lo(.Lb29)
 b .Lpreb
 nop
.Lb29:
 sw $4,116($5)
 addiu $4,$4,2
 lui $9,%hi(.Lb30)
 ori $9,$9,%lo(.Lb30)
 b .Lpreb
 nop
.Lb30:
 sw $4,120($5)
 addiu $4,$4,2
 lui $9,%hi(.Lb31)
 ori $9,$9,%lo(.Lb31)
 b .Lpreb
 nop
.Lb31:
 sw $4,124($5)
 addiu $4,$4,2
 lui $9,%hi(.Lb32)
 ori $9,$9,%lo(.Lb32)
 b .Lpreb
 nop
.Lb32:
 sw $4,128($5)
 addiu $4,$4,2
 lui $9,%hi(.Lb33)
 ori $9,$9,%lo(.Lb33)
 b .Lpreb
 nop
.Lb33:
 sw $4,132($5)
 addiu $4,$4,2
 lui $9,%hi(.Lb34)
 ori $9,$9,%lo(.Lb34)
 b .Lpreb
 nop
.Lb34:
 sw $4,136($5)
 addiu $4,$4,2
 lui $9,%hi(.Lb35)
 ori $9,$9,%lo(.Lb35)
 b .Lpreb
 nop
.Lb35:
 sw $4,140($5)
 addiu $4,$4,2
 lui $9,%hi(.Lb36)
 ori $9,$9,%lo(.Lb36)
 b .Lpreb
 nop
.Lb36:
 sw $4,144($5)
 addiu $4,$4,2
 lui $9,%hi(.Lb37)
 ori $9,$9,%lo(.Lb37)
 b .Lpreb
 nop
.Lb37:
 sw $4,148($5)
 addiu $4,$4,2
 lui $9,%hi(.Lb38)
 ori $9,$9,%lo(.Lb38)
 b .Lpreb
 nop
.Lb38:
 sw $4,152($5)
 addiu $4,$4,2
 lui $9,%hi(.Lb39)
 ori $9,$9,%lo(.Lb39)
 b .Lpreb
 nop
.Lb39:
 sw $4,156($5)
 addiu $4,$4,2
 lui $9,%hi(.Lb40)
 ori $9,$9,%lo(.Lb40)
 b .Lpreb
 nop
.Lb40:
 sw $4,160($5)
 addiu $4,$4,2
 lui $9,%hi(.Lb41)
 ori $9,$9,%lo(.Lb41)
 b .Lpreb
 nop
.Lb41:
 sw $4,164($5)
 addiu $4,$4,2
 lui $9,%hi(.Lb42)
 ori $9,$9,%lo(.Lb42)
 b .Lpreb
 nop
.Lb42:
 sw $4,168($5)
 addiu $4,$4,2
 lui $9,%hi(.Lb43)
 ori $9,$9,%lo(.Lb43)
 b .Lpreb
 nop
.Lb43:
 sw $4,172($5)
 addiu $4,$4,2
 lui $9,%hi(.Lb44)
 ori $9,$9,%lo(.Lb44)
 b .Lpreb
 nop
.Lb44:
 sw $4,176($5)
 addiu $4,$4,2
 lui $9,%hi(.Lb45)
 ori $9,$9,%lo(.Lb45)
 b .Lpreb
 nop
.Lb45:
 sw $4,180($5)
 addiu $4,$4,2
 lui $9,%hi(.Lb46)
 ori $9,$9,%lo(.Lb46)
 b .Lpreb
 nop
.Lb46:
 sw $4,184($5)
 addiu $4,$4,2
 lui $9,%hi(.Lb47)
 ori $9,$9,%lo(.Lb47)
 b .Lpreb
 nop
.Lb47:
 sw $4,188($5)
 addiu $4,$4,2
 lui $9,%hi(.Lb48)
 ori $9,$9,%lo(.Lb48)
 b .Lpreb
 nop
.Lb48:
 sw $4,192($5)
 addiu $4,$4,2
 lui $9,%hi(.Lb49)
 ori $9,$9,%lo(.Lb49)
 b .Lpreb
 nop
.Lb49:
 sw $4,196($5)
 addiu $4,$4,2
 lui $9,%hi(.Lb50)
 ori $9,$9,%lo(.Lb50)
 b .Lpreb
 nop
.Lb50:
 sw $4,200($5)
 addiu $4,$4,2
 lui $9,%hi(.Lb51)
 ori $9,$9,%lo(.Lb51)
 b .Lpreb
 nop
.Lb51:
 sw $4,204($5)
 addiu $4,$4,2
 lui $9,%hi(.Lb52)
 ori $9,$9,%lo(.Lb52)
 b .Lpreb
 nop
.Lb52:
 sw $4,208($5)
 addiu $4,$4,2
 lui $9,%hi(.Lb53)
 ori $9,$9,%lo(.Lb53)
 b .Lpreb
 nop
.Lb53:
 sw $4,212($5)
 addiu $4,$4,2
 lui $9,%hi(.Lb54)
 ori $9,$9,%lo(.Lb54)
 b .Lpreb
 nop
.Lb54:
 sw $4,216($5)
 addiu $4,$4,2
 lui $9,%hi(.Lb55)
 ori $9,$9,%lo(.Lb55)
 b .Lpreb
 nop
.Lb55:
 sw $4,220($5)
 addiu $4,$4,2
 lui $9,%hi(.Lb56)
 ori $9,$9,%lo(.Lb56)
 b .Lpreb
 nop
.Lb56:
 sw $4,224($5)
 addiu $4,$4,2
 lui $9,%hi(.Lb57)
 ori $9,$9,%lo(.Lb57)
 b .Lpreb
 nop
.Lb57:
 sw $4,228($5)
 addiu $4,$4,2
 lui $9,%hi(.Lb58)
 ori $9,$9,%lo(.Lb58)
 b .Lpreb
 nop
.Lb58:
 sw $4,232($5)
 addiu $4,$4,2
 lui $9,%hi(.Lb59)
 ori $9,$9,%lo(.Lb59)
 b .Lpreb
 nop
.Lb59:
 sw $4,236($5)
 addiu $4,$4,2
 lui $9,%hi(.Lb60)
 ori $9,$9,%lo(.Lb60)
 b .Lpreb
 nop
.Lb60:
 sw $4,240($5)
 addiu $4,$4,2
 lui $9,%hi(.Lb61)
 ori $9,$9,%lo(.Lb61)
 b .Lpreb
 nop
.Lb61:
 sw $4,244($5)
 addiu $4,$4,2
 lui $9,%hi(.Lb62)
 ori $9,$9,%lo(.Lb62)
 b .Lpreb
 nop
.Lb62:
 sw $4,248($5)
 addiu $4,$4,2
 lui $9,%hi(.Lb63)
 ori $9,$9,%lo(.Lb63)
 b .Lpreb
 nop
.Lb63:
 sw $4,252($5)
 addiu $4,$4,2
 lui $9,%hi(.Lb64)
 ori $9,$9,%lo(.Lb64)
 b .Lpreb
 nop
.Lb64:
 sw $4,256($5)
 addiu $4,$4,2
 lui $9,%hi(.Lb65)
 ori $9,$9,%lo(.Lb65)
 b .Lpreb
 nop
.Lb65:
 sw $4,260($5)
 addiu $4,$4,2
 lui $9,%hi(.Lb66)
 ori $9,$9,%lo(.Lb66)
 b .Lpreb
 nop
.Lb66:
 sw $4,264($5)
 addiu $4,$4,2
 lui $9,%hi(.Lb67)
 ori $9,$9,%lo(.Lb67)
 b .Lpreb
 nop
.Lb67:
 sw $4,268($5)
 addiu $4,$4,2
 lui $9,%hi(.Lb68)
 ori $9,$9,%lo(.Lb68)
 b .Lpreb
 nop
.Lb68:
 sw $4,272($5)
 addiu $4,$4,2
 lui $9,%hi(.Lb69)
 ori $9,$9,%lo(.Lb69)
 b .Lpreb
 nop
.Lb69:
 sw $4,276($5)
 addiu $4,$4,2
 lui $9,%hi(.Lb70)
 ori $9,$9,%lo(.Lb70)
 b .Lpreb
 nop
.Lb70:
 sw $4,280($5)
 addiu $4,$4,2
 lui $9,%hi(.Lb71)
 ori $9,$9,%lo(.Lb71)
 b .Lpreb
 nop
.Lb71:
 sw $4,284($5)
 addiu $4,$4,2
 lui $9,%hi(.Lb72)
 ori $9,$9,%lo(.Lb72)
 b .Lpreb
 nop
.Lb72:
 sw $4,288($5)
 addiu $4,$4,2
 lui $9,%hi(.Lb73)
 ori $9,$9,%lo(.Lb73)
 b .Lpreb
 nop
.Lb73:
 sw $4,292($5)
 addiu $4,$4,2
 lui $9,%hi(.Lb74)
 ori $9,$9,%lo(.Lb74)
 b .Lpreb
 nop
.Lb74:
 sw $4,296($5)
 addiu $4,$4,2
 lui $9,%hi(.Lb75)
 ori $9,$9,%lo(.Lb75)
 b .Lpreb
 nop
.Lb75:
 sw $4,300($5)
 addiu $4,$4,2
 lui $9,%hi(.Lb76)
 ori $9,$9,%lo(.Lb76)
 b .Lpreb
 nop
.Lb76:
 sw $4,304($5)
 addiu $4,$4,2
 lui $9,%hi(.Lb77)
 ori $9,$9,%lo(.Lb77)
 b .Lpreb
 nop
.Lb77:
 sw $4,308($5)
 addiu $4,$4,2
 lui $9,%hi(.Lb78)
 ori $9,$9,%lo(.Lb78)
 b .Lpreb
 nop
.Lb78:
 sw $4,312($5)
 addiu $4,$4,2
 lui $9,%hi(.Lb79)
 ori $9,$9,%lo(.Lb79)
 b .Lpreb
 nop
.Lb79:
 sw $4,316($5)
 addiu $4,$4,2
 or $31,$8,$0
 jr $31
 nop
.size bounce,.-bounce
.globl _start
_start:
 lui $5,%hi(slots)
 ori $5,$5,%lo(slots)
 addiu $5,$5,4
 li $4,72
 jal bounce
 nop
 li $4,1
 lw $9,-4($5)
 li $10,12
 bne $9,$10,.Lexit
 nop
 lw $9,0($5)
 li $10,72
 bne $9,$10,.Lexit
 nop
 lw $9,4($5)
 li $10,74
 bne $9,$10,.Lexit
 nop
 lw $9,8($5)
 li $10,76
 bne $9,$10,.Lexit
 nop
 lw $9,12($5)
 li $10,78
 bne $9,$10,.Lexit
 nop
 lw $9,16($5)
 li $10,80
 bne $9,$10,.Lexit
 nop
 lw $9,20($5)
 li $10,82
 bne $9,$10,.Lexit
 nop
 lw $9,24($5)
 li $10,84
 bne $9,$10,.Lexit
 nop
 lw $9,28($5)
 li $10,86
 bne $9,$10,.Lexit
 nop
 lw $9,32($5)
 li $10,88
 bne $9,$10,.Lexit
 nop
 lw $9,36($5)
 li $10,90
 bne $9,$10,.Lexit
 nop
 lw $9,40($5)
 li $10,92
 bne $9,$10,.Lexit
 nop
 lw $9,44($5)
 li $10,94
 bne $9,$10,.Lexit
 nop
 lw $9,48($5)
 li $10,96
 bne $9,$10,.Lexit
 nop
 lw $9,52($5)
 li $10,98
 bne $9,$10,.Lexit
 nop
 lw $9,56($5)
 li $10,100
 bne $9,$10,.Lexit
 nop
 lw $9,60($5)
 li $10,102
 bne $9,$10,.Lexit
 nop
 lw $9,64($5)
 li $10,104
 bne $9,$10,.Lexit
 nop
 lw $9,68($5)
 li $10,106
 bne $9,$10,.Lexit
 nop
 lw $9,72($5)
 li $10,108
 bne $9,$10,.Lexit
 nop
 lw $9,76($5)
 li $10,110
 bne $9,$10,.Lexit
 nop
 lw $9,80($5)
 li $10,112
 bne $9,$10,.Lexit
 nop
 lw $9,84($5)
 li $10,114
 bne $9,$10,.Lexit
 nop
 lw $9,88($5)
 li $10,116
 bne $9,$10,.Lexit
 nop
 lw $9,92($5)
 li $10,118
 bne $9,$10,.Lexit
 nop
 lw $9,96($5)
 li $10,120
 bne $9,$10,.Lexit
 nop
 lw $9,100($5)
 li $10,122
 bne $9,$10,.Lexit
 nop
 lw $9,104($5)
 li $10,124
 bne $9,$10,.Lexit
 nop
 lw $9,108($5)
 li $10,126
 bne $9,$10,.Lexit
 nop
 lw $9,112($5)
 li $10,128
 bne $9,$10,.Lexit
 nop
 lw $9,116($5)
 li $10,130
 bne $9,$10,.Lexit
 nop
 lw $9,120($5)
 li $10,132
 bne $9,$10,.Lexit
 nop
 lw $9,124($5)
 li $10,134
 bne $9,$10,.Lexit
 nop
 lw $9,128($5)
 li $10,136
 bne $9,$10,.Lexit
 nop
 lw $9,132($5)
 li $10,138
 bne $9,$10,.Lexit
 nop
 lw $9,136($5)
 li $10,140
 bne $9,$10,.Lexit
 nop
 lw $9,140($5)
 li $10,142
 bne $9,$10,.Lexit
 nop
 lw $9,144($5)
 li $10,144
 bne $9,$10,.Lexit
 nop
 lw $9,148($5)
 li $10,146
 bne $9,$10,.Lexit
 nop
 lw $9,152($5)
 li $10,148
 bne $9,$10,.Lexit
 nop
 lw $9,156($5)
 li $10,150
 bne $9,$10,.Lexit
 nop
 lw $9,160($5)
 li $10,152
 bne $9,$10,.Lexit
 nop
 lw $9,164($5)
 li $10,154
 bne $9,$10,.Lexit
 nop
 lw $9,168($5)
 li $10,156
 bne $9,$10,.Lexit
 nop
 lw $9,172($5)
 li $10,158
 bne $9,$10,.Lexit
 nop
 lw $9,176($5)
 li $10,160
 bne $9,$10,.Lexit
 nop
 lw $9,180($5)
 li $10,162
 bne $9,$10,.Lexit
 nop
 lw $9,184($5)
 li $10,164
 bne $9,$10,.Lexit
 nop
 lw $9,188($5)
 li $10,166
 bne $9,$10,.Lexit
 nop
 lw $9,192($5)
 li $10,168
 bne $9,$10,.Lexit
 nop
 lw $9,196($5)
 li $10,170
 bne $9,$10,.Lexit
 nop
 lw $9,200($5)
 li $10,172
 bne $9,$10,.Lexit
 nop
 lw $9,204($5)
 li $10,174
 bne $9,$10,.Lexit
 nop
 lw $9,208($5)
 li $10,176
 bne $9,$10,.Lexit
 nop
 lw $9,212($5)
 li $10,178
 bne $9,$10,.Lexit
 nop
 lw $9,216($5)
 li $10,180
 bne $9,$10,.Lexit
 nop
 lw $9,220($5)
 li $10,182
 bne $9,$10,.Lexit
 nop
 lw $9,224($5)
 li $10,184
 bne $9,$10,.Lexit
 nop
 lw $9,228($5)
 li $10,186
 bne $9,$10,.Lexit
 nop
 lw $9,232($5)
 li $10,188
 bne $9,$10,.Lexit
 nop
 lw $9,236($5)
 li $10,190
 bne $9,$10,.Lexit
 nop
 lw $9,240($5)
 li $10,192
 bne $9,$10,.Lexit
 nop
 lw $9,244($5)
 li $10,194
 bne $9,$10,.Lexit
 nop
 lw $9,248($5)
 li $10,196
 bne $9,$10,.Lexit
 nop
 lw $9,252($5)
 li $10,198
 bne $9,$10,.Lexit
 nop
 lw $9,256($5)
 li $10,200
 bne $9,$10,.Lexit
 nop
 lw $9,260($5)
 li $10,202
 bne $9,$10,.Lexit
 nop
 lw $9,264($5)
 li $10,204
 bne $9,$10,.Lexit
 nop
 lw $9,268($5)
 li $10,206
 bne $9,$10,.Lexit
 nop
 lw $9,272($5)
 li $10,208
 bne $9,$10,.Lexit
 nop
 lw $9,276($5)
 li $10,210
 bne $9,$10,.Lexit
 nop
 lw $9,280($5)
 li $10,212
 bne $9,$10,.Lexit
 nop
 lw $9,284($5)
 li $10,214
 bne $9,$10,.Lexit
 nop
 lw $9,288($5)
 li $10,216
 bne $9,$10,.Lexit
 nop
 lw $9,292($5)
 li $10,218
 bne $9,$10,.Lexit
 nop
 lw $9,296($5)
 li $10,220
 bne $9,$10,.Lexit
 nop
 lw $9,300($5)
 li $10,222
 bne $9,$10,.Lexit
 nop
 lw $9,304($5)
 li $10,224
 bne $9,$10,.Lexit
 nop
 lw $9,308($5)
 li $10,226
 bne $9,$10,.Lexit
 nop
 lw $9,312($5)
 li $10,228
 bne $9,$10,.Lexit
 nop
 lw $9,316($5)
 li $10,230
 bne $9,$10,.Lexit
 nop
 lw $9,320($5)
 li $10,34
 bne $9,$10,.Lexit
 nop
 li $4,0
.Lexit:
 li $2,5058
 syscall
.data
.align 2
slots: .word 12,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,34
