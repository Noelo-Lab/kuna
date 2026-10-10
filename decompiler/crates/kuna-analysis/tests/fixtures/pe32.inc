; Minimal PE32 (i386) image writer for `nasm -f bin`.
;
; The including file sets IMAGE_DLL (0 or 1), then fills `.text` and
; `.rdata` and defines these labels:
;   entry                         AddressOfEntryPoint
;   import_dir, import_dir_end    IMAGE_IMPORT_DESCRIPTOR array (or both at
;                                 the same place for no imports)
;   iat, iat_end                  the Import Address Table
; It ends with `%include "pe32_end.inc"`.
;
; File layout: headers at 0, .text at RVA 0x1000, .rdata after it.
; FileAlignment 0x200, SectionAlignment 0x1000, image base 0x400000.

bits 32

%define IMAGEBASE 0x00400000
%define RVA(x) ((x) - IMAGEBASE)

section hdr start=0 vstart=IMAGEBASE
mz:
    dw 'MZ'
    times 0x3a db 0
    dd RVA(pe)

pe:
    db 'PE', 0, 0
    dw 0x014c                          ; Machine: i386
    dw 2                               ; NumberOfSections
    dd 0                               ; TimeDateStamp
    dd 0, 0                            ; symbol table
    dw opt_end - opt                   ; SizeOfOptionalHeader
%if IMAGE_DLL
    dw 0x2102                          ; EXECUTABLE | 32BIT | DLL
%else
    dw 0x0102                          ; EXECUTABLE | 32BIT
%endif

opt:
    dw 0x010b                          ; PE32
    db 10, 0                           ; linker version
    dd text_vsize                      ; SizeOfCode
    dd rdata_vsize                     ; SizeOfInitializedData
    dd 0                               ; SizeOfUninitializedData
    dd RVA(entry)                      ; AddressOfEntryPoint
    dd RVA(text_start)                 ; BaseOfCode
    dd RVA(rdata_start)                ; BaseOfData
    dd IMAGEBASE
    dd 0x1000                          ; SectionAlignment
    dd 0x200                           ; FileAlignment
    dw 5, 1, 0, 0                      ; OS / image version
    dw 5, 1                            ; subsystem version
    dd 0                               ; Win32VersionValue
    dd RVA(image_end)                  ; SizeOfImage
    dd 0x200                           ; SizeOfHeaders
    dd 0                               ; CheckSum
    dw 2                               ; Subsystem: Windows GUI
    dw 0                               ; DllCharacteristics
    dd 0x100000, 0x1000, 0x100000, 0x1000
    dd 0                               ; LoaderFlags
    dd 16                              ; NumberOfRvaAndSizes
    dd 0, 0                                        ; export
    dd RVA(import_dir), import_dir_end - import_dir ; import
    times 10 dd 0, 0
    dd RVA(iat), iat_end - iat                     ; IAT
    times 3 dd 0, 0
opt_end:

    db '.text', 0, 0, 0
    dd text_vsize, RVA(text_start)
    dd text_rawsize, section..text.start
    dd 0, 0, 0
    dd 0x60000020                      ; CODE | EXECUTE | READ

    db '.rdata', 0, 0
    dd rdata_vsize, RVA(rdata_start)
    dd rdata_rawsize, section..rdata.start
    dd 0, 0, 0
    dd 0x40000040                      ; INITIALIZED_DATA | READ

    times 0x200 - ($ - $$) db 0

section .text follows=hdr align=0x200 vstart=IMAGEBASE+0x1000
text_start:

section .rdata follows=.text align=0x200 vfollows=.text valign=0x1000
rdata_start:

section .text
