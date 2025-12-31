# Types of Instructions
label?  instruction([nzp]?)  (reg (, reg|label (, reg|label)))? ignore_space

```asm
label? instruction                  ; no operands (includes traps)
label? instruction  reg             ; one register
label? instruction  label           ; one label
label? instruction  reg, reg        ; two registers
label? instruction  reg, label      ; one reg, one register
label? instruction  reg, reg, reg   ; three registers
label? instruction  reg, reg, label ; two registers, one label
label? instruction  reg, reg, imm   ; two registers, one immediate value
```

IMPORTANT VIDEO: https://www.youtube.com/watch?v=oFYZkoglbQk
