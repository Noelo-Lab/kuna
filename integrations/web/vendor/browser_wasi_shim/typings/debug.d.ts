declare class Debug {
    private isEnabled;
    prefix?: string;
    log: (...args: unknown[]) => void;
    constructor(isEnabled: boolean);
    enable(enabled?: boolean): void;
    get enabled(): boolean;
}
export declare const debug: Debug;
export {};
