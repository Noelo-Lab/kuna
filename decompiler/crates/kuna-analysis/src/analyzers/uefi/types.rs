//! EFI table layouts (UEFI 2.10 / PI 1.8), built into kuna `Datatype`s.
//!
//! Each field is spelled in a small C-like vocabulary: a scalar (`UINTN`,
//! `UINT32`, `CHAR16`, ...), an EFI typedef (`EFI_STATUS`, `EFI_HANDLE`, ...), a
//! struct built earlier in [`TABLES`], `VOID`, any of them followed by `*`s, an
//! inline array `T[n]`, or a function pointer `fn RET(ARG, ...)`. Layout follows
//! natural alignment, which is what the UEFI ABI uses on every supported CPU.

use kuna_decomp::dtype::{type_metatype, Datatype, TypeFactory, TypeField};
use kuna_decomp::fspec::PrototypePieces;
use std::collections::BTreeMap;
use std::rc::Rc;

pub(super) const TABLE_HEADER: &[(&str, &str)] = &[
    ("Signature", "UINT64"),
    ("Revision", "UINT32"),
    ("HeaderSize", "UINT32"),
    ("CRC32", "UINT32"),
    ("Reserved", "UINT32"),
];

pub(super) const TABLES: &[(&str, &[(&str, &str)])] = &[
    ("EFI_GUID", &[("Data1", "UINT32"), ("Data2", "UINT16"), ("Data3", "UINT16"), ("Data4", "UINT8[8]")]),
    ("EFI_TABLE_HEADER", TABLE_HEADER),
    ("EFI_CONFIGURATION_TABLE", &[("VendorGuid", "EFI_GUID"), ("VendorTable", "VOID*")]),
    ("EFI_SIMPLE_TEXT_INPUT_PROTOCOL", &[
        ("Reset", "fn EFI_STATUS(VOID*, BOOLEAN)"),
        ("ReadKeyStroke", "fn EFI_STATUS(VOID*, VOID*)"),
        ("WaitForKey", "EFI_EVENT"),
    ]),
    ("EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL", &[
        ("Reset", "fn EFI_STATUS(VOID*, BOOLEAN)"),
        ("OutputString", "fn EFI_STATUS(VOID*, CHAR16*)"),
        ("TestString", "fn EFI_STATUS(VOID*, CHAR16*)"),
        ("QueryMode", "fn EFI_STATUS(VOID*, UINTN, UINTN*, UINTN*)"),
        ("SetMode", "fn EFI_STATUS(VOID*, UINTN)"),
        ("SetAttribute", "fn EFI_STATUS(VOID*, UINTN)"),
        ("ClearScreen", "fn EFI_STATUS(VOID*)"),
        ("SetCursorPosition", "fn EFI_STATUS(VOID*, UINTN, UINTN)"),
        ("EnableCursor", "fn EFI_STATUS(VOID*, BOOLEAN)"),
        ("Mode", "VOID*"),
    ]),
    ("EFI_BOOT_SERVICES", &[
        ("Hdr", "EFI_TABLE_HEADER"),
        ("RaiseTPL", "fn EFI_TPL(EFI_TPL)"),
        ("RestoreTPL", "fn VOID(EFI_TPL)"),
        ("AllocatePages", "fn EFI_STATUS(UINT32, UINT32, UINTN, EFI_PHYSICAL_ADDRESS*)"),
        ("FreePages", "fn EFI_STATUS(EFI_PHYSICAL_ADDRESS, UINTN)"),
        ("GetMemoryMap", "fn EFI_STATUS(UINTN*, VOID*, UINTN*, UINTN*, UINT32*)"),
        ("AllocatePool", "fn EFI_STATUS(UINT32, UINTN, VOID**)"),
        ("FreePool", "fn EFI_STATUS(VOID*)"),
        ("CreateEvent", "fn EFI_STATUS(UINT32, EFI_TPL, VOID*, VOID*, EFI_EVENT*)"),
        ("SetTimer", "fn EFI_STATUS(EFI_EVENT, UINT32, UINT64)"),
        ("WaitForEvent", "fn EFI_STATUS(UINTN, EFI_EVENT*, UINTN*)"),
        ("SignalEvent", "fn EFI_STATUS(EFI_EVENT)"),
        ("CloseEvent", "fn EFI_STATUS(EFI_EVENT)"),
        ("CheckEvent", "fn EFI_STATUS(EFI_EVENT)"),
        ("InstallProtocolInterface", "fn EFI_STATUS(EFI_HANDLE*, EFI_GUID*, UINT32, VOID*)"),
        ("ReinstallProtocolInterface", "fn EFI_STATUS(EFI_HANDLE, EFI_GUID*, VOID*, VOID*)"),
        ("UninstallProtocolInterface", "fn EFI_STATUS(EFI_HANDLE, EFI_GUID*, VOID*)"),
        ("HandleProtocol", "fn EFI_STATUS(EFI_HANDLE, EFI_GUID*, VOID**)"),
        ("Reserved", "VOID*"),
        ("RegisterProtocolNotify", "fn EFI_STATUS(EFI_GUID*, EFI_EVENT, VOID**)"),
        ("LocateHandle", "fn EFI_STATUS(UINT32, EFI_GUID*, VOID*, UINTN*, EFI_HANDLE*)"),
        ("LocateDevicePath", "fn EFI_STATUS(EFI_GUID*, VOID**, EFI_HANDLE*)"),
        ("InstallConfigurationTable", "fn EFI_STATUS(EFI_GUID*, VOID*)"),
        ("LoadImage", "fn EFI_STATUS(BOOLEAN, EFI_HANDLE, VOID*, VOID*, UINTN, EFI_HANDLE*)"),
        ("StartImage", "fn EFI_STATUS(EFI_HANDLE, UINTN*, CHAR16**)"),
        ("Exit", "fn EFI_STATUS(EFI_HANDLE, EFI_STATUS, UINTN, CHAR16*)"),
        ("UnloadImage", "fn EFI_STATUS(EFI_HANDLE)"),
        ("ExitBootServices", "fn EFI_STATUS(EFI_HANDLE, UINTN)"),
        ("GetNextMonotonicCount", "fn EFI_STATUS(UINT64*)"),
        ("Stall", "fn EFI_STATUS(UINTN)"),
        ("SetWatchdogTimer", "fn EFI_STATUS(UINTN, UINT64, UINTN, CHAR16*)"),
        ("ConnectController", "fn EFI_STATUS(EFI_HANDLE, EFI_HANDLE*, VOID*, BOOLEAN)"),
        ("DisconnectController", "fn EFI_STATUS(EFI_HANDLE, EFI_HANDLE, EFI_HANDLE)"),
        ("OpenProtocol", "fn EFI_STATUS(EFI_HANDLE, EFI_GUID*, VOID**, EFI_HANDLE, EFI_HANDLE, UINT32)"),
        ("CloseProtocol", "fn EFI_STATUS(EFI_HANDLE, EFI_GUID*, EFI_HANDLE, EFI_HANDLE)"),
        ("OpenProtocolInformation", "fn EFI_STATUS(EFI_HANDLE, EFI_GUID*, VOID**, UINTN*)"),
        ("ProtocolsPerHandle", "fn EFI_STATUS(EFI_HANDLE, EFI_GUID***, UINTN*)"),
        ("LocateHandleBuffer", "fn EFI_STATUS(UINT32, EFI_GUID*, VOID*, UINTN*, EFI_HANDLE**)"),
        ("LocateProtocol", "fn EFI_STATUS(EFI_GUID*, VOID*, VOID**)"),
        ("InstallMultipleProtocolInterfaces", "fn EFI_STATUS(EFI_HANDLE*, ...)"),
        ("UninstallMultipleProtocolInterfaces", "fn EFI_STATUS(EFI_HANDLE, ...)"),
        ("CalculateCrc32", "fn EFI_STATUS(VOID*, UINTN, UINT32*)"),
        ("CopyMem", "fn VOID(VOID*, VOID*, UINTN)"),
        ("SetMem", "fn VOID(VOID*, UINTN, UINT8)"),
        ("CreateEventEx", "fn EFI_STATUS(UINT32, EFI_TPL, VOID*, VOID*, EFI_GUID*, EFI_EVENT*)"),
    ]),
    ("EFI_RUNTIME_SERVICES", &[
        ("Hdr", "EFI_TABLE_HEADER"),
        ("GetTime", "fn EFI_STATUS(VOID*, VOID*)"),
        ("SetTime", "fn EFI_STATUS(VOID*)"),
        ("GetWakeupTime", "fn EFI_STATUS(BOOLEAN*, BOOLEAN*, VOID*)"),
        ("SetWakeupTime", "fn EFI_STATUS(BOOLEAN, VOID*)"),
        ("SetVirtualAddressMap", "fn EFI_STATUS(UINTN, UINTN, UINT32, VOID*)"),
        ("ConvertPointer", "fn EFI_STATUS(UINTN, VOID**)"),
        ("GetVariable", "fn EFI_STATUS(CHAR16*, EFI_GUID*, UINT32*, UINTN*, VOID*)"),
        ("GetNextVariableName", "fn EFI_STATUS(UINTN*, CHAR16*, EFI_GUID*)"),
        ("SetVariable", "fn EFI_STATUS(CHAR16*, EFI_GUID*, UINT32, UINTN, VOID*)"),
        ("GetNextHighMonotonicCount", "fn EFI_STATUS(UINT32*)"),
        ("ResetSystem", "fn VOID(UINT32, EFI_STATUS, UINTN, VOID*)"),
        ("UpdateCapsule", "fn EFI_STATUS(VOID**, UINTN, EFI_PHYSICAL_ADDRESS)"),
        ("QueryCapsuleCapabilities", "fn EFI_STATUS(VOID**, UINTN, UINT64*, UINT32*)"),
        ("QueryVariableInfo", "fn EFI_STATUS(UINT32, UINT64*, UINT64*, UINT64*)"),
    ]),
    ("EFI_SYSTEM_TABLE", &[
        ("Hdr", "EFI_TABLE_HEADER"),
        ("FirmwareVendor", "CHAR16*"),
        ("FirmwareRevision", "UINT32"),
        ("ConsoleInHandle", "EFI_HANDLE"),
        ("ConIn", "EFI_SIMPLE_TEXT_INPUT_PROTOCOL*"),
        ("ConsoleOutHandle", "EFI_HANDLE"),
        ("ConOut", "EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL*"),
        ("StandardErrorHandle", "EFI_HANDLE"),
        ("StdErr", "EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL*"),
        ("RuntimeServices", "EFI_RUNTIME_SERVICES*"),
        ("BootServices", "EFI_BOOT_SERVICES*"),
        ("NumberOfTableEntries", "UINTN"),
        ("ConfigurationTable", "EFI_CONFIGURATION_TABLE*"),
    ]),
    ("EFI_DXE_SERVICES", &[
        ("Hdr", "EFI_TABLE_HEADER"),
        ("AddMemorySpace", "fn EFI_STATUS(UINT32, EFI_PHYSICAL_ADDRESS, UINT64, UINT64)"),
        ("AllocateMemorySpace", "fn EFI_STATUS(UINT32, UINT32, UINTN, UINT64, EFI_PHYSICAL_ADDRESS*, EFI_HANDLE, EFI_HANDLE)"),
        ("FreeMemorySpace", "fn EFI_STATUS(EFI_PHYSICAL_ADDRESS, UINT64)"),
        ("RemoveMemorySpace", "fn EFI_STATUS(EFI_PHYSICAL_ADDRESS, UINT64)"),
        ("GetMemorySpaceDescriptor", "fn EFI_STATUS(EFI_PHYSICAL_ADDRESS, VOID*)"),
        ("SetMemorySpaceAttributes", "fn EFI_STATUS(EFI_PHYSICAL_ADDRESS, UINT64, UINT64)"),
        ("GetMemorySpaceMap", "fn EFI_STATUS(UINTN*, VOID**)"),
        ("AddIoSpace", "fn EFI_STATUS(UINT32, EFI_PHYSICAL_ADDRESS, UINT64)"),
        ("AllocateIoSpace", "fn EFI_STATUS(UINT32, UINT32, UINTN, UINT64, EFI_PHYSICAL_ADDRESS*, EFI_HANDLE, EFI_HANDLE)"),
        ("FreeIoSpace", "fn EFI_STATUS(EFI_PHYSICAL_ADDRESS, UINT64)"),
        ("RemoveIoSpace", "fn EFI_STATUS(EFI_PHYSICAL_ADDRESS, UINT64)"),
        ("GetIoSpaceDescriptor", "fn EFI_STATUS(EFI_PHYSICAL_ADDRESS, VOID*)"),
        ("GetIoSpaceMap", "fn EFI_STATUS(UINTN*, VOID**)"),
        ("Dispatch", "fn EFI_STATUS()"),
        ("Schedule", "fn EFI_STATUS(EFI_HANDLE, EFI_GUID*)"),
        ("Trust", "fn EFI_STATUS(EFI_HANDLE, EFI_GUID*)"),
        ("ProcessFirmwareVolume", "fn EFI_STATUS(VOID*, UINTN, EFI_HANDLE*)"),
        ("SetMemorySpaceCapabilities", "fn EFI_STATUS(EFI_PHYSICAL_ADDRESS, UINT64, UINT64)"),
    ]),
    ("EFI_PEI_SERVICES", &[
        ("Hdr", "EFI_TABLE_HEADER"),
        ("InstallPpi", "fn EFI_STATUS(VOID**, VOID*)"),
        ("ReInstallPpi", "fn EFI_STATUS(VOID**, VOID*, VOID*)"),
        ("LocatePpi", "fn EFI_STATUS(VOID**, EFI_GUID*, UINTN, VOID**, VOID**)"),
        ("NotifyPpi", "fn EFI_STATUS(VOID**, VOID*)"),
        ("GetBootMode", "fn EFI_STATUS(VOID**, UINT32*)"),
        ("SetBootMode", "fn EFI_STATUS(VOID**, UINT32)"),
        ("GetHobList", "fn EFI_STATUS(VOID**, VOID**)"),
        ("CreateHob", "fn EFI_STATUS(VOID**, UINT16, UINT16, VOID**)"),
        ("FfsFindNextVolume", "fn EFI_STATUS(VOID**, UINTN, VOID**)"),
        ("FfsFindNextFile", "fn EFI_STATUS(VOID**, UINT8, VOID*, VOID**)"),
        ("FfsFindSectionData", "fn EFI_STATUS(VOID**, UINT8, VOID*, VOID**)"),
        ("InstallPeiMemory", "fn EFI_STATUS(VOID**, EFI_PHYSICAL_ADDRESS, UINT64)"),
        ("AllocatePages", "fn EFI_STATUS(VOID**, UINT32, UINTN, EFI_PHYSICAL_ADDRESS*)"),
        ("AllocatePool", "fn EFI_STATUS(VOID**, UINTN, VOID**)"),
        ("CopyMem", "fn VOID(VOID*, VOID*, UINTN)"),
        ("SetMem", "fn VOID(VOID*, UINTN, UINT8)"),
        ("ReportStatusCode", "fn EFI_STATUS(VOID**, UINT32, UINT32, UINT32, EFI_GUID*, VOID*)"),
        ("ResetSystem", "fn EFI_STATUS(VOID**)"),
        ("CpuIo", "VOID*"),
        ("PciCfg", "VOID*"),
        ("FfsFindFileByName", "fn EFI_STATUS(EFI_GUID*, VOID*, VOID**)"),
        ("FfsGetFileInfo", "fn EFI_STATUS(VOID*, VOID*)"),
        ("FfsGetVolumeInfo", "fn EFI_STATUS(VOID*, VOID*)"),
        ("RegisterForShadow", "fn EFI_STATUS(VOID*)"),
        ("FindSectionData3", "fn EFI_STATUS(VOID**, UINT8, UINTN, VOID*, VOID**, UINT32*)"),
        ("FfsGetFileInfo2", "fn EFI_STATUS(VOID*, VOID*)"),
        ("ResetSystem2", "fn VOID(UINT32, EFI_STATUS, UINTN, VOID*)"),
        ("FreePages", "fn EFI_STATUS(VOID**, EFI_PHYSICAL_ADDRESS, UINTN)"),
    ]),
];

/// Builds and caches the EFI types for one image.
pub(super) struct EfiTypes<'a> {
    pub(super) types: &'a dyn TypeFactory,
    pub(super) ptr: i32,
    pub(super) built: BTreeMap<&'static str, Rc<Datatype>>,
}

impl<'a> EfiTypes<'a> {
    /// All of [`TABLES`] under `ptr`-byte pointers, or `None` when a name is
    /// already held by a type of another shape.
    pub(super) fn build(types: &'a dyn TypeFactory, ptr: i32) -> Option<Self> {
        let mut me = EfiTypes { types, ptr, built: BTreeMap::new() };
        for (name, fields) in TABLES {
            let st = me.build_struct(name, fields)?;
            me.built.insert(name, st);
        }
        Some(me)
    }

    pub(super) fn get(&self, name: &str) -> Option<Rc<Datatype>> {
        self.built.get(name).cloned()
    }

    pub(super) fn pointer_to(&self, name: &str) -> Option<Rc<Datatype>> {
        self.types.get_type_pointer(self.ptr, self.get(name)?, 1).ok()
    }

    fn build_struct(&self, name: &str, fields: &[(&str, &str)]) -> Option<Rc<Datatype>> {
        let mut out = Vec::with_capacity(fields.len());
        let mut off = 0i32;
        let mut align = 1i32;
        for (i, (fname, spec)) in fields.iter().enumerate() {
            let ty = self.parse(spec)?;
            let a = natural_align(&ty, self.ptr);
            off = (off + a - 1) / a * a;
            align = align.max(a);
            out.push(TypeField::new(i as i32, off, *fname, ty.clone()));
            off += ty.get_size();
        }
        let size = (off + align - 1) / align * align;
        if let Some(existing) = self.types.find_by_name(name).ok()? {
            return (existing.get_metatype() == type_metatype::TYPE_STRUCT
                && !existing.is_incomplete()
                && existing.get_size() == size)
                .then_some(existing);
        }
        let shell = self.types.get_type_struct(name).ok()?;
        self.types.set_fields_struct_raw(&shell, out, Vec::new(), size, align, 0).ok()
    }

    /// One type in the field vocabulary.
    pub(super) fn parse(&self, spec: &str) -> Option<Rc<Datatype>> {
        let spec = spec.trim();
        if let Some(sig) = spec.strip_prefix("fn ") {
            let pieces = self.pieces("", sig)?;
            let code = self.types.get_type_code_proto(&pieces).or_else(|_| self.types.get_type_code()).ok()?;
            return self.types.get_type_pointer(self.ptr, code, 1).ok();
        }
        if let Some(open) = spec.find('[') {
            let n: i32 = spec[open + 1..spec.len() - 1].parse().ok()?;
            return self.types.get_type_array(n, self.parse(&spec[..open])?).ok();
        }
        if let Some(inner) = spec.strip_suffix('*') {
            let pointee = self.parse(inner)?;
            return self.types.get_type_pointer(self.ptr, pointee, 1).ok();
        }
        let t = self.types;
        let int = |n: i32| t.get_base(n, type_metatype::TYPE_UINT).ok();
        let typedef = |base: Option<Rc<Datatype>>, name: &str| {
            let base = base?;
            t.get_typedef(&base, name, 0, 0).ok().or(Some(base))
        };
        match spec {
            "VOID" => t.get_type_void().ok(),
            "UINT8" => int(1),
            "UINT16" => int(2),
            "UINT32" => int(4),
            "UINT64" => int(8),
            "UINTN" => int(self.ptr),
            "BOOLEAN" => t.get_base(1, type_metatype::TYPE_BOOL).ok(),
            "CHAR16" => t.get_type_char(2).or_else(|_| t.get_base(2, type_metatype::TYPE_UINT)).ok(),
            "CHAR8" => t.get_type_char(1).ok(),
            "EFI_STATUS" => typedef(int(self.ptr), "EFI_STATUS"),
            "EFI_TPL" => typedef(int(self.ptr), "EFI_TPL"),
            "EFI_PHYSICAL_ADDRESS" => typedef(int(8), "EFI_PHYSICAL_ADDRESS"),
            "EFI_HANDLE" | "EFI_EVENT" => {
                let void = t.get_type_pointer(self.ptr, t.get_type_void().ok()?, 1).ok();
                typedef(void, spec)
            }
            other => self.get(other),
        }
    }

    /// `RET(ARG, ...)` as prototype pieces named `name`.
    pub(super) fn pieces(&self, name: &str, sig: &str) -> Option<PrototypePieces> {
        let open = sig.find('(')?;
        let args = sig[open + 1..].strip_suffix(')')?;
        let mut pieces = PrototypePieces { name: name.to_string(), first_var_arg_slot: -1, ..Default::default() };
        pieces.outtype = Some(self.parse(&sig[..open])?);
        for arg in args.split(',').map(str::trim).filter(|a| !a.is_empty()) {
            if arg == "..." {
                pieces.first_var_arg_slot = pieces.intypes.len() as i32;
                continue;
            }
            let (ty, nm) = match arg.rsplit_once(' ') {
                Some((ty, nm)) if !nm.ends_with('*') => (ty, nm),
                _ => (arg, ""),
            };
            pieces.intypes.push(self.parse(ty)?);
            pieces.innames.push(nm.to_string());
        }
        Some(pieces)
    }
}

fn natural_align(ty: &Datatype, ptr: i32) -> i32 {
    match ty.get_metatype() {
        type_metatype::TYPE_STRUCT | type_metatype::TYPE_ARRAY => ty.get_alignment().max(1),
        _ => ty.get_size().clamp(1, 8).min(ptr.max(8)),
    }
}
