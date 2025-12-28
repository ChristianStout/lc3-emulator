.ORIG       x3000

nl          .FILL   #10
char        .FILL   #0

start       GETC
            OUT
            ST      R0, char
            LD      R0, nl
            OUT
            LD      R0, char
            OUT
            LD      R0, nl
            HALT

.END
