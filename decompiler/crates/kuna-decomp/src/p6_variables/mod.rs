//! S6 -- Variable & storage model: HighVariables, merge, stack layout.
//!
//! Stage-aligned module group; declared flatly at the crate root via re-export in
//! `lib.rs` so public paths (`kuna_decomp::<module>`) are unchanged.

pub mod funcdata_facing;
pub mod funcdata_merge;
pub mod funcdata_spacebase;
pub mod cover;
pub mod variable;
pub mod merge;
pub mod kuna_stackalias;
pub mod kuna_stackviews;
pub mod kuna_stackobjects;
pub(crate) mod kuna_stackuses;
pub(crate) mod kuna_savedregisterspills;
pub(crate) mod kuna_stackvaluecopies;
pub(crate) mod kuna_stackvalueorder;
pub(crate) mod kuna_stackvalueviews;
pub(crate) mod kuna_stackloadcover;
pub mod kuna_stackobjectasserts;
pub mod kuna_stackobjectalias;
pub mod kuna_stackparamviews;
pub mod kuna_stackobjectinfo;
pub mod kuna_stackranges;
pub mod kuna_undefname;
pub mod varmap;
pub mod coreaction_cleanup;
pub mod kuna_callretfold;
pub mod kuna_foldcallretphi;
pub mod kuna_indirectonly;
pub mod dynamic;
pub mod kuna_dynamiclocals;
pub mod kuna_registerepochs;
pub mod kuna_dynamichashmax;
pub mod coreaction_stackptr;
pub mod kuna_paramcopyhoist;
pub mod kuna_tiedphitrim;
pub mod kuna_globalvalue;
pub mod kuna_globalorder;
pub mod kuna_calleepop;
pub mod kuna_cookiescramble;
pub mod kuna_nulterminator;
pub mod kuna_endptrbound;
pub(crate) mod kuna_storereach;
pub mod kuna_arrayextent;
pub mod kuna_castobject;
pub mod kuna_impliedrefs;
pub mod kuna_bytehonest;
pub mod kuna_slotptr;
pub mod kuna_hideshadow;
pub mod kuna_pointeevalue;
pub mod kuna_loadorder;

pub(crate) mod kuna_wrappedstackaggregate;
