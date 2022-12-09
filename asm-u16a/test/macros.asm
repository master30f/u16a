%macro memcpy $src,$len,$dest

 mov wax,0                                ; wax will be used as an iterator

loop:

 mov wbx,$src                             ; get the address to read from
 add wax,wbx

 ldr wdx,[wox]                            ; read byte to rd

 mov wbx,$dest                            ; get the address to write to
 add            

 str [wox],wdx                            ; store byte in rd

 inc wax                                  ; increment the iterator

 mov wbx,$len                             ; check check if the iterator is equal to the number of bytes to copy
 sub wbx,wba

 jpz end                                  ; if so, exit the loop

 jmp loop                                 ; else jump back to the start
end:
%~macro

; -------------- ;
; example of use ;
; -------------- ;

%define %counter addr 0x5555,1
main:

 mov addr,'C'                             ; move "COPY ME!" to address 0x5555
 mov addr,'O'         
 mov addr,'P'         
 mov addr,'Y'         
 mov addr,' '         
 mov addr,'M'         
 mov addr,'E'
 mov addr,'!'

 memcpy 0x5555,8,addr                     ; copy the string at position 0x555D

 hlt