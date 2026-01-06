        .ORIG       x3000

        BR          START

hw      .STRINGZ    "Hello, World!\n"

START   LEA         R0, hw  ; Load the address of `hw` into R0
        PUTS                ; Print string to console
        HALT

        .END
