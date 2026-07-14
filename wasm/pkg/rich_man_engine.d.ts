/* tslint:disable */
/* eslint-disable */

export class GameEngine {
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Apply LLM-generated effects to game state.
     * JSON format: {"cashDelta":0,"stocksDelta":0,"cryptoDelta":0,"businessValueDelta":0,"businessCashDelta":0,"proptechDelta":0,"rePriceDelta":0,"log":"","gameOver":false}
     */
    apply_effects(effects_json: string): string;
    generate_share_text(final_total: number): string;
    get_decisions(): string;
    /**
     * Get a compact game context string for LLM prompt
     */
    get_game_context(): string;
    get_glossary(): string;
    get_rank(total: number): string;
    get_state_json(): string;
    load_state_json(json: string): void;
    constructor(capital: number, duration: number, strategy: string);
    process_exit(): string;
    process_year(decision_id: string): string;
    set_seed(seed: bigint): void;
    total_assets(): number;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly __wbg_gameengine_free: (a: number, b: number) => void;
    readonly gameengine_apply_effects: (a: number, b: number, c: number) => [number, number];
    readonly gameengine_generate_share_text: (a: number, b: number) => [number, number];
    readonly gameengine_get_decisions: (a: number) => [number, number];
    readonly gameengine_get_game_context: (a: number) => [number, number];
    readonly gameengine_get_glossary: (a: number) => [number, number];
    readonly gameengine_get_rank: (a: number, b: number) => [number, number];
    readonly gameengine_get_state_json: (a: number) => [number, number];
    readonly gameengine_load_state_json: (a: number, b: number, c: number) => void;
    readonly gameengine_new: (a: number, b: number, c: number, d: number) => number;
    readonly gameengine_process_exit: (a: number) => [number, number];
    readonly gameengine_process_year: (a: number, b: number, c: number) => [number, number];
    readonly gameengine_set_seed: (a: number, b: bigint) => void;
    readonly gameengine_total_assets: (a: number) => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
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
