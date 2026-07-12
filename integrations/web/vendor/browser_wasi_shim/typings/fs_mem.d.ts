import * as wasi from "./wasi_defs.js";
import { Fd, Inode } from "./fd.js";
export declare class OpenFile extends Fd {
    file: File;
    file_pos: bigint;
    constructor(file: File);
    fd_allocate(offset: bigint, len: bigint): number;
    fd_fdstat_get(): {
        ret: number;
        fdstat: wasi.Fdstat | null;
    };
    fd_filestat_set_size(size: bigint): number;
    fd_read(size: number): {
        ret: number;
        data: Uint8Array;
    };
    fd_pread(size: number, offset: bigint): {
        ret: number;
        data: Uint8Array;
    };
    fd_seek(offset: bigint, whence: number): {
        ret: number;
        offset: bigint;
    };
    fd_tell(): {
        ret: number;
        offset: bigint;
    };
    fd_write(data: Uint8Array): {
        ret: number;
        nwritten: number;
    };
    fd_pwrite(data: Uint8Array, offset: bigint): {
        ret: number;
        nwritten: number;
    };
    fd_filestat_get(): {
        ret: number;
        filestat: wasi.Filestat;
    };
}
export declare class OpenDirectory extends Fd {
    dir: Directory;
    constructor(dir: Directory);
    fd_seek(offset: bigint, whence: number): {
        ret: number;
        offset: bigint;
    };
    fd_tell(): {
        ret: number;
        offset: bigint;
    };
    fd_allocate(offset: bigint, len: bigint): number;
    fd_fdstat_get(): {
        ret: number;
        fdstat: wasi.Fdstat | null;
    };
    fd_readdir_single(cookie: bigint): {
        ret: number;
        dirent: wasi.Dirent | null;
    };
    path_filestat_get(flags: number, path_str: string): {
        ret: number;
        filestat: wasi.Filestat | null;
    };
    path_lookup(path_str: string, dirflags: number): {
        ret: number;
        inode_obj: Inode | null;
    };
    path_open(dirflags: number, path_str: string, oflags: number, fs_rights_base: bigint, fs_rights_inheriting: bigint, fd_flags: number): {
        ret: number;
        fd_obj: Fd | null;
    };
    path_create_directory(path: string): number;
    path_link(path_str: string, inode: Inode, allow_dir: boolean): number;
    path_unlink(path_str: string): {
        ret: number;
        inode_obj: Inode | null;
    };
    path_unlink_file(path_str: string): number;
    path_remove_directory(path_str: string): number;
    fd_filestat_get(): {
        ret: number;
        filestat: wasi.Filestat;
    };
    fd_filestat_set_size(size: bigint): number;
    fd_read(size: number): {
        ret: number;
        data: Uint8Array;
    };
    fd_pread(size: number, offset: bigint): {
        ret: number;
        data: Uint8Array;
    };
    fd_write(data: Uint8Array): {
        ret: number;
        nwritten: number;
    };
    fd_pwrite(data: Uint8Array, offset: bigint): {
        ret: number;
        nwritten: number;
    };
}
export declare class PreopenDirectory extends OpenDirectory {
    prestat_name: string;
    constructor(name: string, contents: Map<string, Inode>);
    fd_prestat_get(): {
        ret: number;
        prestat: wasi.Prestat | null;
    };
}
export declare class File extends Inode {
    data: Uint8Array;
    readonly: boolean;
    constructor(data: ArrayBuffer | SharedArrayBuffer | Uint8Array | Array<number>, options?: Partial<{
        readonly: boolean;
    }>);
    path_open(oflags: number, fs_rights_base: bigint, fd_flags: number): {
        ret: number;
        fd_obj: OpenFile;
    };
    get size(): bigint;
    stat(): wasi.Filestat;
}
declare class Path {
    parts: string[];
    is_dir: boolean;
    static from(path: string): {
        ret: number;
        path: Path | null;
    };
    to_path_string(): string;
}
export declare class Directory extends Inode {
    contents: Map<string, Inode>;
    private parent;
    constructor(contents: Map<string, Inode> | [string, Inode][]);
    parent_ino(): bigint;
    path_open(oflags: number, fs_rights_base: bigint, fd_flags: number): {
        ret: number;
        fd_obj: OpenDirectory;
    };
    stat(): wasi.Filestat;
    get_entry_for_path(path: Path): {
        ret: number;
        entry: Inode | null;
    };
    get_parent_dir_and_entry_for_path(path: Path, allow_undefined: boolean): {
        ret: number;
        parent_entry: Directory | null;
        filename: string | null;
        entry: Inode | null;
    };
    create_entry_for_path(path_str: string, is_dir: boolean): {
        ret: number;
        entry: Inode | null;
    };
}
export declare class ConsoleStdout extends Fd {
    private ino;
    write: (buffer: Uint8Array) => void;
    constructor(write: (buffer: Uint8Array) => void);
    fd_filestat_get(): {
        ret: number;
        filestat: wasi.Filestat;
    };
    fd_fdstat_get(): {
        ret: number;
        fdstat: wasi.Fdstat | null;
    };
    fd_write(data: Uint8Array): {
        ret: number;
        nwritten: number;
    };
    static lineBuffered(write: (line: string) => void): ConsoleStdout;
}
export {};
