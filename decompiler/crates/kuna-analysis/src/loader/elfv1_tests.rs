use super::*;

#[test]
fn conflicting_tocs_are_not_arbitrarily_chosen() {
    let mut d=Descriptors(vec![
        Descriptor { address:0x2000,entry:0x1000,toc:0x3000 },
        Descriptor { address:0x2018,entry:0x1000,toc:0x3008 },
        Descriptor { address:0x2030,entry:0x1010,toc:0x3010 },
    ]);
    assert_eq!(d.entry_tocs(),BTreeMap::from([(0x1010,0x3010)]));
    assert_eq!(d.code_address(0x2000),0x1000);
    assert_eq!(d.code_address(0x2008),0x2008);
    d.adjust_vma(0x10000);
    assert_eq!(d.code_address(0x12000),0x11000);
    assert_eq!(d.entry_tocs(),BTreeMap::from([(0x11010,0x13010)]));
}
