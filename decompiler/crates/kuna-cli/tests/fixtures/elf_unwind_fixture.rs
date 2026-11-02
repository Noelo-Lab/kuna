//! Authored ELF: `bump(x)` reads an inline literal and returns `x * 3 + 7`.
//! Compile this file with rustc and pass an output path to regenerate the stage image.

pub const CODE: &[u32] = &[
    0x7c0802a6, 0x429f0009, 7, 0x7d2802a6, 0x81290000, 0x1c630003, 0x7c634a14, 0x7c0803a6,
    0x4e800020,
];

fn put(bytes: &mut [u8], off: usize, width: usize, value: u64, little: bool) {
    let word = if little {
        value.to_le_bytes()
    } else {
        value.to_be_bytes()
    };
    bytes[off..off + width].copy_from_slice(if little {
        &word[..width]
    } else {
        &word[8 - width..]
    });
}

pub fn image(little: bool, wide: bool, pcrel: bool, inner: bool) -> Vec<u8> {
    build(little, wide, pcrel, inner, false)
}

pub fn lsda_image(little: bool, wide: bool, pcrel: bool) -> Vec<u8> {
    build(little, wide, pcrel, false, true)
}

fn build(little: bool, wide: bool, pcrel: bool, inner: bool, lsda: bool) -> Vec<u8> {
    let width = if wide { 8 } else { 4 };
    let mut text = vec![0; CODE.len() * 4];
    for (i, &word) in CODE.iter().enumerate() {
        put(&mut text, i * 4, 4, word.into(), little);
    }
    let mut frame = vec![0; 20];
    put(&mut frame, 0, 4, 16, little);
    frame[8..].copy_from_slice(&[
        1,
        b'z',
        b'R',
        0,
        1,
        0x7c,
        65,
        1,
        if pcrel { 0x1b } else { 0 },
        0,
        0,
        0,
    ]);
    if lsda {
        frame[8..].copy_from_slice(&[
            1,
            b'z',
            b'L',
            b'R',
            0,
            1,
            0x7c,
            65,
            2,
            0x1b,
            if pcrel { 0x1b } else { 0 },
            0,
        ]);
    }
    let field = if pcrel { 4 } else { width };
    let length = (4 + 2 * field + 1 + if lsda { 4 } else { 0 } + 3) & !3;
    frame.resize(20 + 4 + length + 4, 0);
    put(&mut frame, 20, 4, length as u64, little);
    put(&mut frame, 24, 4, 24, little);
    let begin = if pcrel {
        0x1000u32.wrapping_sub(0x201c) as u64
    } else {
        0x1000
    };
    put(&mut frame, 28, field, begin, little);
    put(&mut frame, 28 + field, field, text.len() as u64, little);
    if lsda {
        let off = 28 + field * 2;
        frame[off] = 4;
        put(
            &mut frame,
            off + 1,
            4,
            0x4000u32.wrapping_sub(0x2000 + off as u32 + 1).into(),
            little,
        );
    }
    let syment = if wide { 24 } else { 16 };
    let mut symbols = vec![0; syment * if inner { 3 } else { 2 }];
    for (i, (name, address)) in [(1, 0x1000), (6, 0x100c)].into_iter().enumerate() {
        if i == 1 && !inner {
            break;
        }
        let off = (i + 1) * syment;
        put(&mut symbols, off, 4, name, little);
        if wide {
            symbols[off + 4] = 0x12;
            put(&mut symbols, off + 6, 2, 1, little);
            put(&mut symbols, off + 8, 8, address, little);
            put(&mut symbols, off + 16, 8, (CODE.len() * 4) as u64, little);
        } else {
            put(&mut symbols, off + 4, 4, address, little);
            put(&mut symbols, off + 8, 4, (CODE.len() * 4) as u64, little);
            symbols[off + 12] = 0x12;
            put(&mut symbols, off + 14, 2, 1, little);
        }
    }
    let mut names = vec![0];
    let mut name_offsets = Vec::new();
    for name in [
        ".text",
        ".eh_frame",
        ".symtab",
        ".strtab",
        ".shstrtab",
        ".gcc_except_table",
    ]
    .into_iter()
    .take(if lsda { 6 } else { 5 })
    {
        name_offsets.push(names.len());
        names.extend(name.as_bytes());
        names.push(0);
    }
    let mut sections = vec![
        (1, 6, 0x1000, text, 0, 0, 0),
        (1, 2, 0x2000, frame, 0, 0, 0),
        (2, 0, 0, symbols, 4, 1, syment),
        (3, 0, 0, b"\0bump\0inner\0".to_vec(), 0, 0, 0),
        (3, 0, 0, names, 0, 0, 0),
    ];
    if lsda {
        let mut table = vec![0xff, 0xff, 3, 13];
        table.resize(17, 0);
        put(&mut table, 8, 4, 4, little);
        put(&mut table, 12, 4, 12, little);
        sections.push((1, 2, 0x4000, table, 0, 0, 0));
    }
    let header = if wide { 64 } else { 52 };
    let shsize = if wide { 64 } else { 40 };
    let phsize = if wide { 56 } else { 32 };
    let mut bytes = vec![0; header + phsize * 2];
    let mut offsets = Vec::new();
    for (_, _, _, data, _, _, _) in &sections {
        bytes.resize((bytes.len() + 7) & !7, 0);
        offsets.push(bytes.len());
        bytes.extend(data);
    }
    bytes.resize((bytes.len() + 7) & !7, 0);
    let shoff = bytes.len();
    bytes.resize(shoff + shsize * (sections.len() + 1), 0);
    for (i, (kind, flags, addr, data, link, info, entsize)) in sections.iter().enumerate() {
        let off = shoff + (i + 1) * shsize;
        put(&mut bytes, off, 4, name_offsets[i] as u64, little);
        put(&mut bytes, off + 4, 4, *kind, little);
        put(&mut bytes, off + 8, width, *flags, little);
        put(&mut bytes, off + 8 + width, width, *addr, little);
        put(
            &mut bytes,
            off + 8 + width * 2,
            width,
            offsets[i] as u64,
            little,
        );
        put(
            &mut bytes,
            off + 8 + width * 3,
            width,
            data.len() as u64,
            little,
        );
        put(&mut bytes, off + 8 + width * 4, 4, *link, little);
        put(&mut bytes, off + 12 + width * 4, 4, *info, little);
        put(&mut bytes, off + 16 + width * 4, width, 4, little);
        put(
            &mut bytes,
            off + 16 + width * 5,
            width,
            *entsize as u64,
            little,
        );
    }
    bytes[..7].copy_from_slice(&[
        0x7f,
        b'E',
        b'L',
        b'F',
        if wide { 2 } else { 1 },
        if little { 1 } else { 2 },
        1,
    ]);
    put(&mut bytes, 16, 2, 2, little);
    put(&mut bytes, 18, 2, if wide { 21 } else { 20 }, little);
    put(&mut bytes, 20, 4, 1, little);
    put(&mut bytes, 24, width, 0x1000, little);
    put(
        &mut bytes,
        if wide { 40 } else { 32 },
        width,
        shoff as u64,
        little,
    );
    for i in 0..2 {
        let off = header + i * phsize;
        put(&mut bytes, off, 4, 1, little);
        let flags = if i == 0 { 5 } else { 4 };
        if wide {
            put(&mut bytes, off + 4, 4, flags, little);
            put(&mut bytes, off + 8, 8, offsets[i] as u64, little);
            put(&mut bytes, off + 16, 8, sections[i].2, little);
            put(&mut bytes, off + 24, 8, sections[i].2, little);
            put(&mut bytes, off + 32, 8, sections[i].3.len() as u64, little);
            put(&mut bytes, off + 40, 8, sections[i].3.len() as u64, little);
            put(&mut bytes, off + 48, 8, 4, little);
        } else {
            put(&mut bytes, off + 4, 4, offsets[i] as u64, little);
            put(&mut bytes, off + 8, 4, sections[i].2, little);
            put(&mut bytes, off + 12, 4, sections[i].2, little);
            put(&mut bytes, off + 16, 4, sections[i].3.len() as u64, little);
            put(&mut bytes, off + 20, 4, sections[i].3.len() as u64, little);
            put(&mut bytes, off + 24, 4, flags, little);
            put(&mut bytes, off + 28, 4, 4, little);
        }
    }
    put(
        &mut bytes,
        if wide { 32 } else { 28 },
        width,
        header as u64,
        little,
    );
    let sizes = if wide { 52 } else { 40 };
    for (i, value) in [
        (0, header),
        (1, phsize),
        (2, 2),
        (3, shsize),
        (4, sections.len() + 1),
        (5, 5),
    ] {
        put(&mut bytes, sizes + i * 2, 2, value as u64, little);
    }
    bytes
}

#[allow(dead_code)]
fn main() {
    let output = std::env::args_os().nth(1).expect("output path");
    std::fs::write(output, image(false, false, true, false)).unwrap();
}
