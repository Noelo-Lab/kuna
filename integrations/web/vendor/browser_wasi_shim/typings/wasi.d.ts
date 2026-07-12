import { Fd } from "./fd.js";
export interface Options {
    debug?: boolean;
}
/**
 * An exception that is thrown when the process exits.
 **/
export declare class WASIProcExit extends Error {
    readonly code: number;
    constructor(code: number);
}
export default class WASI {
    args: Array<string>;
    env: Array<string>;
    fds: Array<Fd>;
    inst: {
        exports: {
            memory: WebAssembly.Memory;
        };
    };
    wasiImport: {
        [key: string]: (...args: Array<any>) => unknown;
    };
    start(instance: {
        exports: {
            memory: WebAssembly.Memory;
            _start: () => unknown;
        };
    }): number;
    initialize(instance: {
        exports: {
            memory: WebAssembly.Memory;
            _initialize?: () => unknown;
        };
    }): void;
    constructor(args: Array<string>, env: Array<string>, fds: Array<Fd>, options?: Options);
}
