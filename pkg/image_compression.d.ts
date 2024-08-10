/* tslint:disable */
/* eslint-disable */
/**
* @param {Uint8Array} original_image
* @param {Target} target
* @returns {Uint8Array}
*/
export function compress_image(original_image: Uint8Array, target: Target): Uint8Array;
/**
* Chroma subsampling format
*/
export enum ChromaSampling {
/**
* Both vertically and horizontally subsampled.
*/
  Cs420 = 0,
/**
* Horizontally subsampled.
*/
  Cs422 = 1,
/**
* Not subsampled.
*/
  Cs444 = 2,
/**
* Monochrome.
*/
  Cs400 = 3,
}
/**
*/
export class Target {
  free(): void;
/**
* @param {number} width
* @param {number} height
* @param {number} filter
* @param {number} quality
* @param {bigint} max_alloc
* @param {number} max_width
* @param {number} max_height
*/
  constructor(width: number, height: number, filter: number, quality: number, max_alloc: bigint, max_width: number, max_height: number);
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_target_free: (a: number) => void;
  readonly target_new: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => number;
  readonly compress_image: (a: number, b: number, c: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
