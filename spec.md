# Memory Specification

2^16 2B RAM


# Processor Specification

## Registers

 - IP     *Instruction pointer*
 - 1111 WVX *Immediate value word register*
    - 1101 BVX
    - 1110 BVY
 - 0011 WAX *General purpose word register A*
    - 0001 BAX
    - 0010 BAY
 - 0111 WBX *General purpose word register B*
    - 0101 BBX
    - 0110 BBY
 - 1011 WOX *Output word register*
    - 1001 BOX
    - 1010 BOY

x1 x2 y1 y2

x:
 - 00 A
 - 01 B
 - 10 O
 - 11 V
y:
 - XOR(y1, y2) B
 - AND(y1, y2) W
 - y2=1        X
 - y2=0        Y

# Instruction Set

### Instruction format
```
[cccc iiii aaaa bbbb vvvv vvvv vvvv vvvv]
 ^-------^ ^-------^ ^-----------------^
 |         |         Immediate value [WORD]
 |         Arguments [BYTE]
 Instruction code [BYTE]
```

 - c: Category
 - i: Instruction
 - a: First argument (Sink)
 - b: Second argument (Source)
 - v: Immediate value

#### Instruction code
Must be present.
Represented by an array of 1s and 0s.


#### Arguments
May be present.
Represented by an array of x's, y's and U's.
 - x: First argument
 - y: Second argument
 - U: Undefined -- may be anything


#### Immediate value
May be present. Can only be 1st argument.
Represented by an array of i's.


## Control Flow

 - `0000 0000                                  NOP`  *No-op*
 - `0000 0001  xxxx UUUU  iiii iiii iiii iiii  JMP`  *Jumps to address in register $x*
 - `0000 0010  xxxx yyyy  iiii iiii iiii iiii  JPZ`  *Jumps to address in register $x if register $y is zero*
 - `0000 0011  xxxx yyyy  iiii iiii iiii iiii  JPN`  *Jumps to address in register $x if register $y is negative*
 - `0000 0100                                  HLT`  *Halts the CPU until next interrupt*


## Arithmetics

 - `0001 0000  xxxx yyyy  iiii iiii iiii iiii  ADD`  *Adds $x and $y together and stores the result in WOX*
 - `0001 0001  xxxx yyyy  iiii iiii iiii iiii  SUB`  *Subtracts register $y from register $x and stores the result in WOX*
 - `0001 0010  xxxx UUUU                       INC`  *Increments register $x by one*
 - `0001 0011  xxxx UUUU                       DEC`  *Decrements register $x by one*
 - `0001 0100  xxxx UUUU                       NEG`  *Negates register $x*


## Logic

 - `0010 0000  xxxx yyyy  iiii iiii iiii iiii  OR `  *Applies logical OR operation on $x and $y and stores the result in WOX*
 - `0010 0001  xxxx yyyy  iiii iiii iiii iiii  AND`  *Applies logical AND operation on $x and $y and stores the result in WOX*
 - `0010 0010  xxxx yyyy  iiii iiii iiii iiii  XOR`  *Applies logical XOR operation on $x and $y and stores the result in WOX*
 - `0010 0011  xxxx UUUU  iiii iiii iiii iiii  NOT`  *Applies logical NOT operation on $x and stores the result in WOX*


## Registers

 - `0011 0000  xxxx yyyy  iiii iiii iiii iiii  MOV`  *Moves value in register $y to register $x*


## Memory Management

 - `0100 0000  xxxx yyyy  iiii iiii iiii iiii  LDR`  *Loads a value from memory at address in register $y to register $x*
 - `0100 0001  xxxx yyyy  iiii iiii iiii iiii  STR`  *Stores a value to memory at address in register $x from register $y*