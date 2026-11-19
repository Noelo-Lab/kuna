use super::*;

#[test]
fn names_and_orders() {
    assert_eq!(intrinsic("vpaddd_avx2", 32), Some(("_mm256_add_epi32".into(), vec![0, 1])));
    assert_eq!(intrinsic("vpmaxub_avx2", 32), Some(("_mm256_max_epu8".into(), vec![0, 1])));
    assert_eq!(intrinsic("vpshufb_avx2", 32), Some(("_mm256_shuffle_epi8".into(), vec![0, 1])));
    assert_eq!(intrinsic("vpunpcklbw_avx2", 32), Some(("_mm256_unpacklo_epi8".into(), vec![0, 1])));
    assert_eq!(intrinsic("vaddps_avx512f", 64), Some(("_mm512_add_ps".into(), vec![0, 1])));
    assert_eq!(intrinsic("vpand_avx2", 32), Some(("_mm256_and_si256".into(), vec![0, 1])));
    assert_eq!(intrinsic("vfmadd132ps_fma", 32), Some(("_mm256_fmadd_ps".into(), vec![0, 2, 1])));
    assert_eq!(intrinsic("vfmadd213ps_fma", 16), Some(("_mm_fmadd_ps".into(), vec![1, 0, 2])));
    assert_eq!(intrinsic("vfnmsub231sd_fma", 16), Some(("_mm_fnmsub_sd".into(), vec![1, 2, 0])));
}

#[test]
fn masked_forms_take_the_mask_infix() {
    assert_eq!(intrinsic("vaddps_avx512f_mask", 64), Some(("_mm512_mask_add_ps".into(), vec![0, 1, 2, 3])));
    assert_eq!(intrinsic("vaddps_avx512f_maskz", 64), Some(("_mm512_maskz_add_ps".into(), vec![0, 1, 2])));
}

#[test]
fn unmapped_user_ops_keep_their_names() {
    assert_eq!(intrinsic("vpaddd_avx2", 8), None);
    assert_eq!(intrinsic("cpuid", 16), None);
    assert_eq!(intrinsic("vzeroupper_avx", 16), None);
    assert_eq!(intrinsic("vpslld_avx2", 32), None);
    assert_eq!(intrinsic("vpermd_avx2", 32), None);
}
