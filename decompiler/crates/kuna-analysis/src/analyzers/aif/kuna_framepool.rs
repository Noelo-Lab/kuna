//! Frame-prefix reconciliation uses literal evidence from the original walk and
//! validated frames before deciding whether an earlier prefix is code.

use super::*;

pub(super) fn policy(
    arch: &kuna_decomp::architecture::Architecture,
    code_space: Rc<AddrSpace>,
) -> GapWalk<'_> {
    GapWalk {
        strict: arch.analysis_aifstrict,
        corroborate: arch.analysis_aifcorroborate,
        bracket: None,
        noppad: false,
        poolslide: arch.analysis_aifpoolslide.then_some((arch, code_space)),
    }
}

pub(super) fn literals<'a>(
    listing: &Listing,
    decoder: &GapDecoder,
    frames: &BTreeMap<u64, u32>,
    policy: Option<(&'a kuna_decomp::architecture::Architecture, Rc<AddrSpace>)>,
) -> Option<kuna_aifpoolslide::LiteralWords<'a>> {
    let (arch, code_space) = policy?;
    let mut literals = kuna_aifpoolslide::LiteralWords::from_listing(
        listing,
        kuna_poolentry::DecodeMode { arch, code_space },
    );
    literals.add_instructions(listing, decoder, frames.keys().copied());
    Some(literals)
}
