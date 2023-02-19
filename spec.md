3 registers
**IP** *Instruction pointer*
**BX** *General purpose byte register X*
**BY** *General purpose byte register Y*

0000 0000             **NOOP** *No-op*
0000 0001  pppp pppp  **JUMP** *Puts value $p in register IP*

0000 0010             **INCX** *Increments value in register BX by one*
0000 0011             **DECX** *Decrements value in register BX by one*
0000 0100  pppp pppp  **SETX** *Sets value in register BX to $p*
0000 0101  pppp pppp  **STOX** *Stores value in register BX to RAM on address $p*
0000 0110  pppp pppp  **LDAX** *Loads value to register BX from RAM on address $p*
0000 0111  pppp pppp  **JPZX** *Jumps to address $p if the value in register BX is zero*

0000 1000             **INCY** *Increments value in register BY by one*
0000 1001             **DECY** *Decrements value in register BY by one*
0000 1010  pppp pppp  **SETY** *Sets value in register BY to $p*
0000 1011  pppp pppp  **STOY** *Stores value in register BY to RAM on address $p*
0000 1100  pppp pppp  **LDAY** *Loads value to register BY from RAM on address $p*
0000 1101  pppp pppp  **JPZY** *Jumps to address $p if the value in register BY is zero*

0000 0000 - 1110 1111
1111 0000 - 1111 1111