/* tslint:disable */
/* eslint-disable */
export class ImageProcessor {
  free(): void;
  constructor(data: Uint8Array);
  to_bytes(): Uint8Array;
  width(): number;
  height(): number;
  grayscale(): void;
  rotate90(): void;
  flip_vertical(): void;
  flip_horizontal(): void;
  adjust_brightness(value: number): void;
  adjust_contrast(contrast: number): void;
  blur(sigma: number): void;
  sharpen(sigma: number, threshold: number): void;
  detect_edges(): void;
  generate_thumbnail(max_width: number, max_height: number): Uint8Array;
  to_png(): Uint8Array;
  to_jpeg(quality: number): Uint8Array;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_imageprocessor_free: (a: number, b: number) => void;
  readonly imageprocessor_new: (a: number, b: number) => [number, number, number];
  readonly imageprocessor_to_bytes: (a: number) => [number, number];
  readonly imageprocessor_width: (a: number) => number;
  readonly imageprocessor_height: (a: number) => number;
  readonly imageprocessor_grayscale: (a: number) => void;
  readonly imageprocessor_rotate90: (a: number) => void;
  readonly imageprocessor_flip_vertical: (a: number) => void;
  readonly imageprocessor_flip_horizontal: (a: number) => void;
  readonly imageprocessor_adjust_brightness: (a: number, b: number) => void;
  readonly imageprocessor_adjust_contrast: (a: number, b: number) => void;
  readonly imageprocessor_blur: (a: number, b: number) => void;
  readonly imageprocessor_sharpen: (a: number, b: number, c: number) => void;
  readonly imageprocessor_detect_edges: (a: number) => void;
  readonly imageprocessor_generate_thumbnail: (a: number, b: number, c: number) => [number, number];
  readonly imageprocessor_to_png: (a: number) => [number, number, number, number];
  readonly imageprocessor_to_jpeg: (a: number, b: number) => [number, number, number, number];
  readonly __wbindgen_export_0: WebAssembly.Table;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __externref_table_dealloc: (a: number) => void;
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
