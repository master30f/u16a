3 registers
**IP** *Instruction pointer*
**RX** *General purpose byte register X*
**RY** *General purpose byte register Y*

00 0000 0000             **NOOP** *No-op*
01 0000 0001  pppp pppp  **JUMP** *Puts value $p in register IP*

02 0000 0010             **INCX** *Increments value in register BX by one*
03 0000 0011             **DECX** *Decrements value in register BX by one*
04 0000 0100  pppp pppp  **SETX** *Sets value in register BX to $p*
05 0000 0101  pppp pppp  **STRX** *Stores value in register BX to RAM on address $p*
06 0000 0110  pppp pppp  **LDRX** *Loads value to register BX from RAM on address $p*
07 0000 0111  pppp pppp  **JPZX** *Jumps to address $p if the value in register BX is zero*
 
08 0000 1000             **INCY** *Increments value in register BY by one*
09 0000 1001             **DECY** *Decrements value in register BY by one*
0a 0000 1010  pppp pppp  **SETY** *Sets value in register BY to $p*
0b 0000 1011  pppp pppp  **STRY** *Stores value in register BY to RAM on address $p*
0c 0000 1100  pppp pppp  **LDRY** *Loads value to register BY from RAM on address $p*
0d 0000 1101  pppp pppp  **JPZY** *Jumps to address $p if the value in register BY is zero*
 
0e 0000 1110             **CPXY** **
0f 0000 1111             **CPYX** **
10 0001 0000             **CPXZ** **
11 0001 0001             **CPYZ** **
12 0001 0010             **CPZX** **
13 0001 0011             **CPZY** **

14 0001 0100             **ADDX** **
15 0001 0101             **ADDY** **
16 0001 0110             **SBXY** **
17 0001 0111             **SBYX** **
18 0001 1000             **OR**   **
19 0001 1001             **AND**  **

0000 0000 - 1110 1111
1111 0000 - 1111 1111