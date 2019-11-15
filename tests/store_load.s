# Basic lw and beq tests

.data
array:
.word   1  255    1024   0xcafebabe
array2: .word   1,255,    1024,   0xcafebabe

.text
main:   
    li  $v0, 0x1000 # $v0  = 0x1000 testing li
                      
    lw  $v1, 12($v0)    # $v1  = 0xcafebabe 
    slt $t0, $v1, $0    # $t0  = 1      testing slt
    slt $t1, $v0, $0    # $t1  = 0
    slt $t2, $0, $v0    # $t2 = 1
    slt $t3, $v0, $v1   # $t3 = 0

    bne $t1, $0, end
    beq $t3, $t0, target

    lbu $a0, 12($v0)    # $a0 = be      test load byte unsigned
    lbu $a1, 13($v0)    # $a1 = ba
    lbu $a2, 14($v0)    # $a2 = fe
    lbu $a3, 15($v0)    # $a3 = ca

    sw  $0, 0($v0)  #           test stores
    lw  $t4, 0($v0) # $t4 = 0
    sb  $a0, 2($v0) 
    lw  $t5, 0($v0) # $t5 = 0x00be0000


    la  $t6, target     # $t6 = target = 0x0000005c
    jr  $t6         #  PC = 0x0000005c  test indirect branches

skipped:
    addi    $t7, $zero, 1   # skipped so $t7 remains 0
    j   skipped         # skipped

end:    lui $s1, 0xf00f # $s1 = 0xf00f0000 testing lui
    jr $ra

target:
    addi    $s0, $zero, 2   # $s0 = 2
    bne $s0, $0, end    # unconditional jump

