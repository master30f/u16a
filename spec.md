4 registers
**IP** *Instruction pointer*
**BX** *General purpose byte register X*
**BY** *General purpose byte register Y*

0000 0000             **NOOP** *No-op*
0000 0001  pppp pppp  **JUMP** *Puts value $p in register IP*
0000 0010             **INCX** *Increments value in register BX by one*
0000 0011             **DECX** *fdf*
0000 0100  pppp pppp  **SETX** *dasdas*
0000 0101  pppp pppp  **STOX** *Stores value in register BX to RAM on address $p*
0000 0110  pppp pppp  **LDAX** *fdsf*
0000 0111  pppp pppp  **JPZX** *Jumps to address $p if the value in register X is zero*

0000 0000 - 1110 1111
1111 0000 - 1111 1111