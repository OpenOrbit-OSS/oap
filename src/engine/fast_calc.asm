; OAP/src/engine/fast_calc.asm
; Low-level vector operations for emergency orbital maneuvers.
; Target: ARM64 / AArch64 (Common in modern satellite OBCs)

section .text
global vec3_add_fast

; Function: vec3_add_fast
; Description: Adds two 3D vectors (f64) with minimal latency.
; Arguments: X0 (ptr to Vec A), X1 (ptr to Vec B), X2 (ptr to Result)
vec3_add_fast:
    LDP D0, D1, [X0]         ; Load A.x and A.y into D0, D1
    LDR D2, [X0, #16]        ; Load A.z into D2
    
    LDP D3, D4, [X1]         ; Load B.x and B.y into D3, D4
    LDR D5, [X1, #16]        ; Load B.z into D5

    FADD D0, D0, D3          ; D0 = A.x + B.x
    FADD D1, D1, D4          ; D1 = A.y + B.y
    FADD D2, D2, D5          ; D2 = A.z + B.z

    STP D0, D1, [X2]         ; Store result x and y
    STR D2, [X2, #16]        ; Store result z
    RET