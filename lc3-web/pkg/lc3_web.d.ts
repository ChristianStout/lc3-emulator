/* tslint:disable */
/* eslint-disable */
export interface DebugIO {
    input_stream: string[];
    output_stream: string[];
}

export interface Lc3IO {
    target: IOTarget;
}

export interface Memory {
    inner: number[];
}

export interface Registers {
    r: [number, number, number, number, number, number, number, number];
    pc: number;
    n: boolean;
    z: boolean;
    p: boolean;
    ir: number;
    psr: number;
    halt: boolean;
}

export interface Token {
    inner_token: TokenType;
    to: number;
    from: number;
    file_relative_to: number;
    file_relative_from: number;
    line_num: number;
    original_match: string;
}

export interface TokenCollection {
    tokens: Token[];
}

export interface VM {
    instructions: Record<number, Instruction>;
    registers: Registers;
    memory: Memory;
    io: Lc3IO;
}

export interface WebIO {
    output_stream: string[];
    input_stream: string[];
}

export type Add = null;

export type And = null;

export type Br = null;

export type Directive = "ORIG" | "FILL" | "BLKW" | "STRINGZ" | "END";

export type JmpRet = null;

export type Jsr = null;

export type Ld = null;

export type Ldi = null;

export type Ldr = null;

export type Lea = null;

export type Not = null;

export type OpcodeIns = "Add" | "And" | { Br: [boolean, boolean, boolean] } | "Jmp" | "Jsr" | "Jsrr" | "Ld" | "Ldi" | "Ldr" | "Lea" | "Not" | "Ret" | "Rti" | "St" | "Sti" | "Str" | { Trap: number } | "Reserved" | "INVALID";

export type Rti = null;

export type St = null;

export type Sti = null;

export type Str = null;

export type TokenType = { Label: string } | { Instruction: OpcodeIns } | { Directive: Directive } | { Number: number } | { String: string } | { Register: number } | { INVALID: string };

export type Trap = null;


export class WebVM {
    free(): void;
    [Symbol.dispose](): void;
    get_ir_value_as_hex(): Promise<string>;
    get_pc(): number;
    get_pc_value_as_hex(): Promise<string>;
    get_reg_value_as_hex(reg_value: number): Promise<string>;
    is_awaiting_input(): Promise<boolean>;
    is_halted(): boolean;
    load_into_memory(file: Uint16Array): void;
    mem_get(loc: number): number;
    constructor();
    reset_machine(): Promise<void>;
    set_awaiting_input(is: boolean): Promise<void>;
    set_pc(new_pc: number): void;
    set_reg(reg: number, value: number): Promise<void>;
    step(): Promise<void>;
}

export function assemble(file: string): Uint16Array | undefined;

export function get_tokens(file: string): TokenCollection;

export function highlight_text(text: string): string;

export function make_memory_table(): Promise<void>;

export function pop_from_input_stream(): string | undefined;

export function push_char_to_output(c: string): void;

export function u16_to_ascii_rep(n: number): string;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly pop_from_input_stream: () => number;
    readonly push_char_to_output: (a: number) => void;
    readonly webio_new: () => any;
    readonly assemble: (a: number, b: number) => [number, number];
    readonly make_memory_table: () => any;
    readonly u16_to_ascii_rep: (a: number) => [number, number];
    readonly highlight_text: (a: number, b: number) => [number, number];
    readonly __wbg_webvm_free: (a: number, b: number) => void;
    readonly webvm_get_ir_value_as_hex: (a: number) => any;
    readonly webvm_get_pc: (a: number) => number;
    readonly webvm_get_pc_value_as_hex: (a: number) => any;
    readonly webvm_get_reg_value_as_hex: (a: number, b: number) => any;
    readonly webvm_is_awaiting_input: (a: number) => any;
    readonly webvm_is_halted: (a: number) => number;
    readonly webvm_load_into_memory: (a: number, b: number, c: number) => void;
    readonly webvm_mem_get: (a: number, b: number) => number;
    readonly webvm_new: () => number;
    readonly webvm_reset_machine: (a: number) => any;
    readonly webvm_set_awaiting_input: (a: number, b: number) => any;
    readonly webvm_set_pc: (a: number, b: number) => void;
    readonly webvm_set_reg: (a: number, b: number, c: number) => any;
    readonly webvm_step: (a: number) => any;
    readonly get_tokens: (a: number, b: number) => any;
    readonly wasm_bindgen__closure__destroy__h526b5303e2c7d6e8: (a: number, b: number) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h6ac4f0b2d2f61cf4: (a: number, b: number, c: any, d: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h6753d1b549cb6a6f: (a: number, b: number, c: any) => void;
    readonly __wbindgen_malloc_command_export: (a: number, b: number) => number;
    readonly __wbindgen_realloc_command_export: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_exn_store_command_export: (a: number) => void;
    readonly __externref_table_alloc_command_export: () => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_free_command_export: (a: number, b: number, c: number) => void;
    readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
