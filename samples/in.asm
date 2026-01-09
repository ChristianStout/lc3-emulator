.orig x3000

                br          begin

prompt          .stringz    "\nwill you give this repo a star? (y/n) > "

begin           lea         r0, prompt
                in
                out
                br          calc

char_y          .fill       #121
char_n          .fill       #110

calc            ld          r1, char_y
                not         r1, r1
                add         r1, r1, #1
                add         r1, r1, r0
                brz         thank
                ld          r1, char_n
                not         r1, r1
                add         r1, r1, #1
                add         r1, r1, r0
                brz         scold

                lea         r0, hmm
                puts
                br          begin
hmm             .stringz    "\n?"

thx_msg         .stringz    "\nwow, tysm :) <3\n"
thank           lea         r0, thx_msg
                puts
                halt

bad_msg         .stringz    "\nhow dare you"
scold           lea         r0, bad_msg
                puts
                br          begin
                halt

.end
