import * as wasi from "./wasi_defs.js";
export declare abstract class Fd {
    fd_allocate(offset: bigint, len: bigint): number;
    fd_close(): number;
    fd_fdstat_get(): {
        ret: number;
        fdstat: wasi.Fdstat | null;
    };
    fd_fdstat_set_flags(flags: number): number;
    fd_fdstat_set_rights(fs_rights_base: bigint, fs_rights_inheriting: bigint): number;
    fd_filestat_get(): {
        ret: number;
        filestat: wasi.Filestat | null;
    };
    fd_filestat_set_size(size: bigint): number;
    fd_filestat_set_times(atim: bigint, mtim: bigint, fst_flags: number): number;
    fd_pread(size: number, offset: bigint): {
        ret: number;
        data: Uint8Array;
    };
    fd_prestat_get(): {
        ret: number;
        prestat: wasi.Prestat | null;
    };
    fd_pwrite(data: Uint8Array, offset: bigint): {
        ret: number;
        nwritten: number;
    };
    fd_read(size: number): {
        ret: number;
        data: Uint8Array;
    };
    fd_readdir_single(cookie: bigint): {
        ret: number;
        dirent: wasi.Dirent | null;
    };
    fd_seek(offset: bigint, whence: number): {
        ret: number;
        offset: bigint;
    };
    fd_sync(): number;
    fd_tell(): {
        ret: number;
        offset: bigint;
    };
    fd_write(data: Uint8Array): {
        ret: number;
        nwritten: number;
    };
    path_create_directory(path: string): number;
    path_filestat_get(flags: number, path: string): {
        ret: number;
        filestat: wasi.Filestat | null;
    };
    path_filestat_set_times(flags: number, path: string, atim: bigint, mtim: bigint, fst_flags: number): number;
    path_link(path: string, inode: Inode, allow_dir: boolean): number;
    path_unlink(path: string): {
        ret: number;
        inode_obj: Inode | null;
    };
    path_lookup(path: string, dirflags: number): {
        ret: number;
        inode_obj: Inode | null;
    };
    path_open(dirflags: number, path: string, oflags: number, fs_rights_base: bigint, fs_rights_inheriting: bigint, fd_flags: number): {
        ret: number;
        fd_obj: Fd | null;
    };
    path_readlink(path: string): {
        ret: number;
        data: string | null;
    };
    path_remove_directory(path: string): number;
    path_rename(old_path: string, new_fd: number, new_path: string): number;
    path_unlink_file(path: string): number;
}
export declare abstract class Inode {
    ino: bigint;
    constructor();
    private static next_ino;
    static issue_ino(): bigint;
    static root_ino(): bigint;
    abstract path_open(oflags: number, fs_rights_base: bigint, fd_flags: number): {
        ret: number;
        fd_obj: Fd | null;
    };
    abstract stat(): wasi.Filestat;
}
