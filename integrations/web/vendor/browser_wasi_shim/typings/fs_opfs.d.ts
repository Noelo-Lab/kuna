import * as wasi from "./wasi_defs.js";
import { Fd, Inode } from "./fd.js";
export interface FileSystemSyncAccessHandle {
    close(): void;
    flush(): void;
    getSize(): number;
    read(buffer: ArrayBuffer | ArrayBufferView, options?: {
        at: number;
    }): number;
    truncate(to: number): void;
    write(buffer: ArrayBuffer | ArrayBufferView, options?: {
        at: number;
    }): number;
}
export declare class SyncOPFSFile extends Inode {
    handle: FileSystemSyncAccessHandle;
    readonly: boolean;
    constructor(handle: FileSystemSyncAccessHandle, options?: Partial<{
        readonly: boolean;
    }>);
    path_open(oflags: number, fs_rights_base: bigint, fd_flags: number): {
        ret: number;
        fd_obj: OpenSyncOPFSFile;
    };
    get size(): bigint;
    stat(): wasi.Filestat;
}
export declare class OpenSyncOPFSFile extends Fd {
    file: SyncOPFSFile;
    position: bigint;
    ino: bigint;
    constructor(file: SyncOPFSFile);
    fd_allocate(offset: bigint, len: bigint): number;
    fd_fdstat_get(): {
        ret: number;
        fdstat: wasi.Fdstat | null;
    };
    fd_filestat_get(): {
        ret: number;
        filestat: wasi.Filestat;
    };
    fd_filestat_set_size(size: bigint): number;
    fd_read(size: number): {
        ret: number;
        data: Uint8Array;
    };
    fd_seek(offset: number | bigint, whence: number): {
        ret: number;
        offset: bigint;
    };
    fd_write(data: Uint8Array): {
        ret: number;
        nwritten: number;
    };
    fd_sync(): number;
}
