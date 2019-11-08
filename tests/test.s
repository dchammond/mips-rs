.text
main:
	li $t0, 1
	add $t0, $t0, $0
	beq $t0, $0, func
	jr $ra
func:
	li $t1, 1
	jr $ra
