.ORIG       x0000

            BR      start

nl          .FILL   #10
char        .FILL   #0
A           .FILL   #65

start       LD      R0, A
            OUT
            ST      R0, char
            LD      R0, nl
            OUT
            LD      R0, char
            OUT
            LD      R0, nl
            PUTSP
            HALT
            BR      start

.END
