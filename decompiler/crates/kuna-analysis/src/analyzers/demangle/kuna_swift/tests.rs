//! (kuna `swiftdemangle`) The demangler's pinned corpus: real symbols from
//! the Swift 6.3 stdlib / Foundation and iOS app binaries, each with the
//! full form `swift-demangle --compact` prints for it and the name-only
//! form kuna installs.

use super::*;

/// (mangled, `swift-demangle` full form, kuna name-only form).
const CORPUS: &[(&str, &str, &str)] = &[
    (
        "$sSS5countSivg",
        "Swift.String.count.getter : Swift.Int",
        "Swift.String.count.getter",
    ),
    (
        "$sSS5countSivpMV",
        "property descriptor for Swift.String.count : Swift.Int",
        "Swift.String.count.property_descriptor",
    ),
    (
        "$ss5print_9separator10terminatoryypd_S2StF",
        "Swift.print(_: Any..., separator: Swift.String, terminator: Swift.String) -> ()",
        "Swift.print(_:separator:terminator:)",
    ),
    (
        "_$s10Foundation10URLRequestV3url11cachePolicy15timeoutIntervalAcA3URLV_So017NSURLRequestCacheE0VSdtcfC",
        "Foundation.URLRequest.init(url: Foundation.URL, cachePolicy: __C.NSURLRequestCachePolicy, timeoutInterval: Swift.Double) -> Foundation.URLRequest",
        "Foundation.URLRequest.init(url:cachePolicy:timeoutInterval:)",
    ),
    (
        "_$sSo9OS_os_logC0B0E9subsystem8categoryABSS_SStcfC",
        "(extension in os):__C.OS_os_log.init(subsystem: Swift.String, category: Swift.String) -> __C.OS_os_log",
        "__C.OS_os_log.init(subsystem:category:)",
    ),
    (
        "_$sSD10FoundationE19_bridgeToObjectiveCSo12NSDictionaryCyF",
        "(extension in Foundation):Swift.Dictionary._bridgeToObjectiveC() -> __C.NSDictionary",
        "Swift.Dictionary._bridgeToObjectiveC()",
    ),
    (
        "_$s25SkyFloatingLabelTextField0abcdE8WithIconC06createG033_CCDD58451C60914C988B087785AA6D6FLLyyF",
        "SkyFloatingLabelTextField.SkyFloatingLabelTextFieldWithIcon.(createIcon in _CCDD58451C60914C988B087785AA6D6F)() -> ()",
        "SkyFloatingLabelTextField.SkyFloatingLabelTextFieldWithIcon.createIcon()",
    ),
    (
        "_$s6Apollo16URLSessionClientC7sessionSo12NSURLSessionCSgvM",
        "Apollo.URLSessionClient.session.modify : __C.NSURLSession?",
        "Apollo.URLSessionClient.session.modify",
    ),
    (
        "_$s7RxSwift14ObservableTypeP0A5CocoaE4bind2toqd__qd__xXE_tlF",
        "(extension in RxCocoa):RxSwift.ObservableType.bind<A>(to: (A) -> A1) -> A1",
        "RxSwift.ObservableType.bind(to:)",
    ),
    (
        "_$sSL2geoiySbx_xtFZTj",
        "dispatch thunk of static Swift.Comparable.>= infix(A, A) -> Swift.Bool",
        "Swift.Comparable.>=(_:_:).dispatch_thunk",
    ),
    (
        "_$s10Foundation4DateV4fromACs7Decoder_p_tKcfC",
        "Foundation.Date.init(from: Swift.Decoder) throws -> Foundation.Date",
        "Foundation.Date.init(from:)",
    ),
    (
        "_$s7SwiftUI7ForEachVA2A4ViewR0_rlE_2id7contentACyxq_q0_Gx_s7KeyPathCy7ElementQzq_Gq0_AKctcfC",
        "(extension in SwiftUI):SwiftUI.ForEach< where C: SwiftUI.View>.init(_: A, id: Swift.KeyPath<A.Element, B>, content: (A.Element) -> C) -> SwiftUI.ForEach<A, B, C>",
        "SwiftUI.ForEach.init(_:id:content:)",
    ),
    (
        "_$s10Foundation11JSONDecoderC6decode_4fromxxm_AA4DataVtKSeRzlFTj",
        "dispatch thunk of Foundation.JSONDecoder.decode<A where A: Swift.Decodable>(_: A.Type, from: Foundation.Data) throws -> A",
        "Foundation.JSONDecoder.decode(_:from:).dispatch_thunk",
    ),
    (
        "_$s7SwiftUI22UIHostingConfigurationV15makeContentView5UIKit09UIContentG0_So6UIViewCXcyF",
        "SwiftUI.UIHostingConfiguration.makeContentView() -> __C.UIView & UIKit.UIContentView",
        "SwiftUI.UIHostingConfiguration.makeContentView()",
    ),
    (
        "_$s8Dispatch1poiyAA0A4TimeVAD_AA0aB8IntervalOtF",
        "Dispatch.+ infix(Dispatch.DispatchTime, Dispatch.DispatchTimeInterval) -> Dispatch.DispatchTime",
        "Dispatch.+(_:_:)",
    ),
    (
        "_$sSa034_makeUniqueAndReserveCapacityIfNotB0yyFyXl_Ts5",
        "generic pre-specialization <Swift.AnyObject> of Swift.Array._makeUniqueAndReserveCapacityIfNotUnique() -> ()",
        "Swift.Array._makeUniqueAndReserveCapacityIfNotUnique().generic_prespecialization<Swift.AnyObject>",
    ),
    (
        "_$sSo17NSKeyedUnarchiverC10FoundationE16unarchivedObject9ofClasses4fromypSgSayyXlXpG_AC4DataVtKFZ",
        "static (extension in Foundation):__C.NSKeyedUnarchiver.unarchivedObject(ofClasses: [Swift.AnyObject.Type], from: Foundation.Data) throws -> Any?",
        "__C.NSKeyedUnarchiver.unarchivedObject(ofClasses:from:)",
    ),
    (
        "$ss17CodingUserInfoKeyVSYsSY8rawValuexSg03RawF0Qz_tcfCTW",
        "protocol witness for Swift.RawRepresentable.init(rawValue: A.RawValue) -> A? in conformance Swift.CodingUserInfoKey : Swift.RawRepresentable in Swift",
        "<Swift.CodingUserInfoKey:Swift.RawRepresentable>.init(rawValue:)",
    ),
    (
        "_$s6Sentry0A38WatchdogTerminationAttributesProcessorC7setUseryySo0aG0CSgFyAFcfU_TA",
        "partial apply forwarder for closure #1 (__C.SentryUser) -> () in Sentry.SentryWatchdogTerminationAttributesProcessor.setUser(__C.SentryUser?) -> ()",
        "Sentry.SentryWatchdogTerminationAttributesProcessor.setUser(_:).closure#1.partial_apply",
    ),
    (
        "_$sSo10AVFileTypea12VisionCameraE10withStringABSS_tKcfCTf4nd_g",
        "function signature specialization <Arg[1] = Dead, Return = Owned To Guaranteed> of (extension in VisionCamera):__C.AVFileType.init(withString: Swift.String) throws -> __C.AVFileType",
        "__C.AVFileType.init(withString:).function_signature_specialization",
    ),
    (
        "$s22_FoundationCollections5DequeV8IteratorV4nextxSgyFxAC13_UnsafeHandleVyx_GXEfU_TA",
        "partial apply forwarder for closure #1 (_FoundationCollections.Deque<A>._UnsafeHandle) -> A in _FoundationCollections.Deque.Iterator.next() -> A?",
        "_FoundationCollections.Deque.Iterator.next().closure#1.partial_apply",
    ),
    (
        "$s20FoundationEssentials9JSONErrorOWOy",
        "outlined copy of FoundationEssentials.JSONError",
        "FoundationEssentials.JSONError.outlined_copy",
    ),
    (
        "$ss5Int32V12SIMD4StorageVyABSicipADTK",
        "key path getter for Swift.Int32.SIMD4Storage.subscript(Swift.Int) -> Swift.Int32 : Swift.Int32.SIMD4Storage",
        "Swift.Int32.SIMD4Storage.subscript(_:).keypath_getter",
    ),
    (
        "_$s6Sentry0A14RRWebMetaEventCfD",
        "Sentry.SentryRRWebMetaEvent.__deallocating_deinit",
        "Sentry.SentryRRWebMetaEvent.__deallocating_deinit",
    ),
    (
        "$ss26_KeyedDecodingContainerBoxCyAByxGxcfCTq",
        "method descriptor for Swift._KeyedDecodingContainerBox.__allocating_init(A) -> Swift._KeyedDecodingContainerBox<A>",
        "Swift._KeyedDecodingContainerBox.__allocating_init(_:).method_descriptor",
    ),
    (
        "$ss5SliceVsSMRzrlEyAByxGSny5IndexQzGcig",
        "(extension in Swift):Swift.Slice< where A: Swift.MutableCollection>.subscript.getter : (Swift.Range<A.Index>) -> Swift.Slice<A>",
        "Swift.Slice.subscript(_:).getter",
    ),
    (
        "$ss7UnicodeO15GeneralCategoryO11otherNumberyA2DmFWC",
        "enum case for Swift.Unicode.GeneralCategory.otherNumber(Swift.Unicode.GeneralCategory.Type) -> Swift.Unicode.GeneralCategory",
        "Swift.Unicode.GeneralCategory.otherNumber(_:).enum_case",
    ),
    (
        "$s20FoundationEssentials11IANACharsetV9iso2022JP_WZTv_",
        "outlined variable #0 of one-time initialization function for iso2022JP",
        "FoundationEssentials.IANACharset.iso2022JP.once_function.outlined_variable_0",
    ),
    (
        "_$sypSgIegr_yXlSgIeyBa_TR",
        "reabstraction thunk helper from @escaping @callee_guaranteed () -> (@out Any?) to @escaping @callee_unowned @convention(block) () -> (@autoreleased Swift.AnyObject?)",
        "reabstraction_thunk_helper<()->(Any?),()->(Swift.AnyObject?)>",
    ),
    (
        "_$sSh8_VariantV6insertySb8inserted_x17memberAfterInserttxnFSDyS2SG_Tg5",
        "generic specialization <[Swift.String : Swift.String]> of Swift.Set._Variant.insert(__owned A) -> (inserted: Swift.Bool, memberAfterInsert: A)",
        "Swift.Set._Variant.insert(_:).generic_specialization<[Swift.String:Swift.String]>",
    ),
    (
        "$ss10_NativeSetV7element2atxSh5IndexVyx_G_tFTm",
        "merged Swift._NativeSet.element(at: Swift.Set<A>.Index) -> A",
        "Swift._NativeSet.element(at:).merged",
    ),
    (
        "$ss20_ConcreteHashableBoxVWV",
        "value witness table for Swift._ConcreteHashableBox",
        "Swift._ConcreteHashableBox.value_witness_table",
    ),
    (
        "_$sSo17OS_dispatch_queueCMaTm",
        "merged type metadata accessor for __C.OS_dispatch_queue",
        "__C.OS_dispatch_queue.type_metadata_accessor.merged",
    ),
    (
        "$ss21_ClosureBasedSequenceVMn",
        "nominal type descriptor for Swift._ClosureBasedSequence",
        "Swift._ClosureBasedSequence.nominal_type_descriptor",
    ),
    (
        "_$s9CryptoKit6SHA256VAA12HashFunctionAAMc",
        "protocol conformance descriptor for CryptoKit.SHA256 : CryptoKit.HashFunction in CryptoKit",
        "<CryptoKit.SHA256:CryptoKit.HashFunction>.protocol_conformance_descriptor",
    ),
    (
        "_$sScTss5NeverORs_rlE5valuexvgTu",
        "async function pointer to (extension in Swift):Swift.Task< where B == Swift.Never>.value.getter : A",
        "Swift.Task.value.getter.async_function_pointer",
    ),
    (
        "$sIeAgH_s5Error_pIegHzo_TRTATQ0_",
        "(1) await resume partial function for partial apply forwarder for reabstraction thunk helper from @escaping @isolated(any) @callee_guaranteed @async () -> () to @escaping @callee_guaranteed @async () -> (@error @owned Swift.Error)",
        "reabstraction_thunk_helper<()->(),()->(Swift.Error)>.partial_apply.await_resume_1",
    ),
    (
        "$ss22_AbstractStringStorageP02asB0SSvgTj",
        "dispatch thunk of Swift._AbstractStringStorage.asString.getter : Swift.String",
        "Swift._AbstractStringStorage.asString.getter.dispatch_thunk",
    ),
    (
        "_$s6Sentry0A14ProfileOptionsC16profileAppStartsSbvsTq",
        "method descriptor for Sentry.SentryProfileOptions.profileAppStarts.setter : Swift.Bool",
        "Sentry.SentryProfileOptions.profileAppStarts.setter.method_descriptor",
    ),
    (
        "$s20FoundationEssentials14DateComponentsV11_weekOfYearSiSgvpfi",
        "variable initialization expression of FoundationEssentials.DateComponents._weekOfYear : Swift.Int?",
        "FoundationEssentials.DateComponents._weekOfYear.initializer",
    ),
    (
        "$ss20_ArrayBufferProtocolPsE22_arrayOutOfPlaceUpdateyys011_ContiguousaB0Vy7ElementQzGz_S2iySpyAGG_SitXEtFfA2_",
        "default argument 3 of (extension in Swift):Swift._ArrayBufferProtocol._arrayOutOfPlaceUpdate(inout Swift._ContiguousArrayBuffer<A.Element>, Swift.Int, Swift.Int, (Swift.UnsafeMutablePointer<A.Element>, Swift.Int) -> ()) -> ()",
        "Swift._ArrayBufferProtocol._arrayOutOfPlaceUpdate(_:_:_:_:).default_argument#3",
    ),
    (
        "$ss11GlobalActorPsE14assertIsolated_4file4lineySSyXK_s12StaticStringVSutFZfA_SSycfu_",
        "implicit closure #1 () -> Swift.String in default argument 0 of static (extension in Swift):Swift.GlobalActor.assertIsolated(_: @autoclosure () -> Swift.String, file: Swift.StaticString, line: Swift.UInt) -> ()",
        "Swift.GlobalActor.assertIsolated(_:file:line:).default_argument#0.implicit_closure#1",
    ),
    (
        "_$s7Combine10SubscriberMp",
        "protocol descriptor for Combine.Subscriber",
        "Combine.Subscriber.protocol_descriptor",
    ),
    (
        "_$sSo10SentryUserC0A0E10CodingKeysOAEs0C3KeyACWl",
        "lazy protocol witness table accessor for type (extension in Sentry):__C.SentryUser.CodingKeys and conformance (extension in Sentry):__C.SentryUser.CodingKeys : Swift.CodingKey in Sentry",
        "<__C.SentryUser.CodingKeys:Swift.CodingKey>.lazy_protocol_witness_table_accessor",
    ),
    (
        "_$s6Sentry0A18UserFeedbackWidgetCMm",
        "metaclass for Sentry.SentryUserFeedbackWidget",
        "Sentry.SentryUserFeedbackWidget.metaclass",
    ),
    (
        "$ss15__VaListBuilderCMo",
        "class metadata base offset for Swift.__VaListBuilder",
        "Swift.__VaListBuilder.class_metadata_base_offset",
    ),
    (
        "_$s12VisionCamera18OrientationManagerC06sensorC033_A39332CD6E0CDA719B494061DA21A62DLLAA0C0OvW",
        "VisionCamera.OrientationManager.(sensorOrientation in _A39332CD6E0CDA719B494061DA21A62D).didset : VisionCamera.Orientation",
        "VisionCamera.OrientationManager.sensorOrientation.didset",
    ),
    (
        "$s20FoundationEssentials4DataV13SearchOptionsV8anchoredAEvau",
        "FoundationEssentials.Data.SearchOptions.anchored.unsafeMutableAddressor : FoundationEssentials.Data.SearchOptions",
        "FoundationEssentials.Data.SearchOptions.anchored.unsafeMutableAddressor",
    ),
    (
        "$ss5Int64VSLsSL2leoiySbx_xtFZTW",
        "protocol witness for static Swift.Comparable.<= infix(A, A) -> Swift.Bool in conformance Swift.Int64 : Swift.Comparable in Swift",
        "<Swift.Int64:Swift.Comparable>.<=(_:_:)",
    ),
    (
        "$ss18AdditiveArithmeticPsE1popyxxFZ",
        "static (extension in Swift):Swift.AdditiveArithmetic.+ prefix(A) -> A",
        "Swift.AdditiveArithmetic.+(_:)",
    ),
    (
        "$sSD4KeysVyxq__GSlsSl30_customIndexOfEquatableElementy0C0QzSgSg0F0QzFTW",
        "protocol witness for Swift.Collection._customIndexOfEquatableElement(A.Element) -> A.Index?? in conformance [A : B].Keys : Swift.Collection in Swift",
        "<[A:B].Keys:Swift.Collection>._customIndexOfEquatableElement(_:)",
    ),
    (
        "$s20FoundationEssentials20PredicateExpressionsO9TypeCheckVMF",
        "reflection metadata field descriptor FoundationEssentials.PredicateExpressions.TypeCheck",
        "FoundationEssentials.PredicateExpressions.TypeCheck.reflection_field_descriptor",
    ),
    (
        "_$sSo11NSPredicateC10FoundationE6format_ABSSh_s7CVarArg_pdtcfC",
        "(extension in Foundation):__C.NSPredicate.init(format: __shared Swift.String, _: Swift.CVarArg...) -> __C.NSPredicate",
        "__C.NSPredicate.init(format:_:)",
    ),
    (
        "_$s6Sentry0A12EnvelopeItemC4data10Foundation4DataVSgvpWvd",
        "direct field offset for Sentry.SentryEnvelopeItem.data : Foundation.Data?",
        "Sentry.SentryEnvelopeItem.data.field_offset",
    ),
    (
        "_$s12VisionCamera0B4ViewC6minFpsSo8NSNumberCSgvsTo",
        "@objc VisionCamera.CameraView.minFps.setter : __C.NSNumber?",
        "VisionCamera.CameraView.minFps.setter.objc",
    ),
    (
        "$ss26AnyBidirectionalCollectionV5countSivpMV",
        "property descriptor for Swift.AnyBidirectionalCollection.count : Swift.Int",
        "Swift.AnyBidirectionalCollection.count.property_descriptor",
    ),
    (
        "_TtC6Toyota24AddMileageViewController",
        "Toyota.AddMileageViewController",
        "Toyota.AddMileageViewController",
    ),
    (
        "_TtCC8DGCharts26HorizontalBarChartRendererP33_AAB8A416B07B7ABA6A1148A4C00D2BA76Buffer",
        "DGCharts.HorizontalBarChartRenderer.(Buffer in _AAB8A416B07B7ABA6A1148A4C00D2BA7)",
        "DGCharts.HorizontalBarChartRenderer.Buffer",
    ),
    (
        "_TtGC7SwiftUI19UIHostingControllerVVS_7TabItem8RootView_",
        "SwiftUI.UIHostingController<SwiftUI.TabItem.RootView>",
        "SwiftUI.UIHostingController<SwiftUI.TabItem.RootView>",
    ),
    (
        "_TtP6FRAuth15DeviceCollector_",
        "FRAuth.DeviceCollector",
        "FRAuth.DeviceCollector",
    ),
    (
        "_TtCs12_SwiftObject",
        "Swift._SwiftObject",
        "Swift._SwiftObject",
    ),
];

#[test]
fn full_form_matches_swift_demangle() {
    for (raw, full, _) in CORPUS {
        assert_eq!(demangle_swift_full(raw).as_deref(), Some(*full), "{raw}");
    }
}

#[test]
fn name_only_form() {
    for (raw, _, name) in CORPUS {
        assert_eq!(demangle_swift_name(raw).as_deref(), Some(*name), "{raw}");
        assert!(
            !name.contains(' '),
            "{raw}: name-only forms never contain a space"
        );
    }
}

/// Parse-or-passthrough: everything outside the coverage answers `None`,
/// byte-identical passthrough being the caller's contract.
#[test]
fn out_of_coverage_names_pass_through() {
    for raw in [
        // not Swift at all
        "main",
        "_ZN4core9panicking5panic17h0123456789abcdefE",
        "$LN3",
        "_RNvCs1234_4core5panic",
        // truncated / malformed
        "$s",
        "$s99zz",
        "$s4main5PointVMc",
        "$s4main5PointVQQ",
        // autodiff (not implemented)
        "$s16_Differentiation13AnyDerivativeV1poiyA2C_ACtFZTJfSSUpSr",
        // an identifier count past the end
        "$s4main99helloyyF",
        // a substitution that does not exist yet
        "$sAAyyF",
        // old mangling outside the runtime-name subset
        "_TtCZFO6Toyota19PackageConfigurator24didFinishLaunchingPrefixFT_T_L_23AppFeatureContainerGate",
        "_TtGC7SwiftUI19UIHostingController",
        "_TtC1",
    ] {
        assert_eq!(demangle_swift_full(raw), None, "{raw} must pass through");
        assert_eq!(demangle_swift_name(raw), None, "{raw} must pass through");
    }
}

/// The spellings of one body grammar: `$s`, `$S`, the Mach-O leading
/// underscore, and the Swift-4 `_T0` prefix.
#[test]
fn prefix_spellings() {
    for raw in [
        "$s4main5helloyyF",
        "_$s4main5helloyyF",
        "$S4main5helloyyF",
        "_T04main5helloyyF",
        "__T04main5helloyyF",
    ] {
        assert_eq!(
            demangle_swift_name(raw).as_deref(),
            Some("main.hello()"),
            "{raw}"
        );
        assert_eq!(
            demangle_swift_full(raw).as_deref(),
            Some("main.hello() -> ()"),
            "{raw}"
        );
    }
}

/// Word substitutions (`0…`) and punycode (`00…`) identifiers.
#[test]
fn identifier_encodings() {
    // `main` and `AbcDefGHI` register the words main, Abc, Def, GHI;
    // `02Myac1_B0` reuses them (`a` = main, `c` = Def, `B` = Abc).
    assert_eq!(
        demangle_swift_name("$s4main9AbcDefGHIV02Myac1_B0V"),
        Some("main.AbcDefGHI.MymainDef_Abc".into())
    );
    // The ABI document's example: `vergüenza`.
    assert_eq!(
        demangle_swift_name("$s4main0012vergenza_JFaV"),
        Some("main.vergüenza".into())
    );
    // Non-identifier ASCII inside a punycoded C++ template name travels in
    // the surrogate range.
    assert_eq!(
        demangle_swift_full("$sSo0041shared_ptrmargelonitroAnyMap_ieBCkfaEDbycV"),
        Some("__C.shared_ptr<margelo.nitro.AnyMap>".into())
    );
}

/// Every demangle of a mutated corpus entry returns (no panic, bounded time).
#[test]
fn mutated_inputs_never_panic() {
    let mut seed: u64 = 0x9e37_79b9_7f4a_7c15;
    let mut rnd = || {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        seed
    };
    let alphabet = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789_$.";
    for _ in 0..20_000 {
        let mut s = CORPUS[(rnd() % CORPUS.len() as u64) as usize]
            .0
            .as_bytes()
            .to_vec();
        for _ in 0..=(rnd() % 4) {
            let pos = (rnd() % s.len() as u64) as usize;
            match rnd() % 3 {
                0 => s.insert(pos, alphabet[(rnd() % alphabet.len() as u64) as usize]),
                1 => {
                    s.remove(pos);
                }
                _ => s[pos] = alphabet[(rnd() % alphabet.len() as u64) as usize],
            }
            if s.is_empty() {
                break;
            }
        }
        let s = String::from_utf8(s).unwrap();
        let _ = demangle_swift_full(&s);
        let _ = demangle_swift_name(&s);
    }
    // Repeat counts and lengths are bounded.
    assert_eq!(
        demangle_swift_full(&format!("$s{}", "S2000a".repeat(100))),
        None
    );
    assert_eq!(
        demangle_swift_full(&format!("$s4main3foo{}F", "y".repeat(100_000))),
        None
    );
}

/// Length fields and indexes near the integer limits, and unbounded nesting:
/// every one returns `None` instead of overflowing or exhausting the stack.
#[test]
fn hostile_counts_and_nesting_are_refused() {
    let huge = u64::MAX.to_string();
    let near = (u64::MAX - 1).to_string();
    for raw in [
        // identifier lengths past the end of the text and past usize
        format!("$s{huge}main"),
        format!("$s{near}main"),
        format!("_TtC{huge}main"),
        format!("_TtC{near}main3Foo"),
        // a C-type length
        format!("$sXzC{huge}abc"),
        // a punycode length
        format!("$s00{huge}_abc"),
        // substitution / repeat counts past the table
        format!("$s4mainA{huge}_"),
        format!("$s4mainA{near}_"),
        format!("$sS{huge}i"),
        // dropped-argument counts
        format!("$s4main3fooyyFTt{huge}g5"),
        // a builtin width
        format!("$sBi{huge}_"),
        // an over-limit repeat count is not silently ignored
        "$sSDyS2147483648SGSgMD".to_string(),
        "$s5XCAPP6RegionV4codeS4294967296Svg".to_string(),
        // an over-limit dropped-argument count
        "$s4main3fooyyFSi_Tt2147483648g5".to_string(),
    ] {
        assert_eq!(demangle_swift_full(&raw), None, "{raw}");
        assert_eq!(demangle_swift_name(&raw), None, "{raw}");
    }
    // Counts above the reference demangler's `i32::MAX` limit are refused
    // even where the text could satisfy them.
    assert_eq!(demangle_swift_full("$s4main3fooL2147483648_yyF"), None);
    assert_eq!(
        demangle_swift_full("$s4main3fooL2147483647_yyF").as_deref(),
        Some("foo #2147483649 () -> () in main")
    );
    assert_eq!(
        demangle_swift_name("$s4main3fooL2147483647_yyF").as_deref(),
        Some("main.foo#2147483649()")
    );
    // Unbounded nesting in the runtime-name grammar and in the `$s` grammar.
    assert_eq!(
        demangle_swift_full(&format!("_Tt{}C4main3Foo", "G".repeat(100_000))),
        None
    );
    assert_eq!(
        demangle_swift_full(&format!("_Tt{}C4main3Foo", "C".repeat(100_000))),
        None
    );
    assert_eq!(
        demangle_swift_full(&format!("$s1a{}N", "1bV".repeat(30_000))),
        None
    );
    assert_eq!(
        demangle_swift_full(&format!(
            "$s1a{}y{}GN",
            "1bV".repeat(20_000),
            "Si_".repeat(20_000)
        )),
        None
    );
    assert_eq!(
        demangle_swift_full(&format!("$s4main3FooV{}N", "Sg".repeat(30_000))),
        None
    );
}
