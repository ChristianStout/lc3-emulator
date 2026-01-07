;; LC-3 OS - Trap Vector Table
;; ---------------------------
;;
;; The Trap Vector Table is a part of the LC-3 that simply jumps
;; to the internal functions that execute a given trap.
;;
;; Traps may have dedicated keywords, but they are in
;; fact just normal LC-3 functions that are run in privelidged
;; mode.

.ORIG       x0020

            
