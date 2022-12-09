.version 1

%include std

.section program
main:
 mov wax,string
 mov wbx,stdout
 mov wcx,sys_write
 int call_kernel

.section data
string:
@sstr "Hello, World!"