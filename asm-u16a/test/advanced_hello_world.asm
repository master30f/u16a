%define VGA    0x5555
%define offset %counter 0,1

main:
 mov VGA+offset,72
 mov VGA+offset,101
 mov VGA+offset,108
 mov VGA+offset,108
 mov VGA+offset,111
 mov VGA+offset,44
 mov VGA+offset,32
 mov VGA+offset,87
 mov VGA+offset,111
 mov VGA+offset,114
 mov VGA+offset,108
 mov VGA+offset,100
 hlt