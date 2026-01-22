;; The operating system is an LC-3 program that runs in priveledged
;; mode in the LC-3.

.ORIG                       x0020

;;;     TRAP VECTOR TABLE
;;; -------------------------
                            BR      GETC_FN
                            BR      OUT_FN
                            BR      PUTS_FN
                            BR      IN_FN
                            BR      PUTSP_FN
                            BR      HALT_FN

;;;    STATUS REGISTER LOCATIONS
;;; -------------------------------
KEYBOARD_STATUS_REGISTER    .FILL   xFE00
KEYBOARD_DATA_REGISTER      .FILL   xFE02
DISPLAY_STATUS_REGISTER     .FILL   xFE04
DISPLAY_DATA_REGISTER       .FILL   xFE06
MACHINE_CONTROL_REGISTER    .FILL   xFFFE

HALT_FN                     LDI     R5, MACHINE_CONTROL_REGISTER


                            RET

.END
