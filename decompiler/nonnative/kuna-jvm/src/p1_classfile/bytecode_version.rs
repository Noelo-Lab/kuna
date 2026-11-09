//! Port of `org/jetbrains/java/decompiler/code/BytecodeVersion.java`.

use crate::rt::{JavaEq, JavaHash};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct BytecodeVersion {
    pub major: i32,
    pub minor: i32,
}

impl BytecodeVersion {
    pub const PREVIEW: i32 = 65535;
    pub const MAJOR_1_0_2: i32 = 45;
    pub const MAJOR_1_2: i32 = 46;
    pub const MAJOR_1_3: i32 = 47;
    pub const MAJOR_1_4: i32 = 48;
    pub const MAJOR_5: i32 = 49;
    pub const MAJOR_6: i32 = 50;
    pub const MAJOR_7: i32 = 51;
    pub const MAJOR_8: i32 = 52;
    pub const MAJOR_9: i32 = 53;
    pub const MAJOR_10: i32 = 54;
    pub const MAJOR_11: i32 = 55;
    pub const MAJOR_12: i32 = 56;
    pub const MAJOR_13: i32 = 57;
    pub const MAJOR_14: i32 = 58;
    pub const MAJOR_15: i32 = 59;
    pub const MAJOR_16: i32 = 60;
    pub const MAJOR_17: i32 = 61;
    pub const MAJOR_18: i32 = 62;
    pub const MAJOR_19: i32 = 63;
    pub const MAJOR_20: i32 = 64;
    pub const MAJOR_21: i32 = 65;

    pub fn new(major: i32, minor: i32) -> BytecodeVersion {
        BytecodeVersion {
            major: major & 0xffff,
            minor: minor & 0xffff,
        }
    }

    pub fn has_enums(&self) -> bool {
        self.major >= Self::MAJOR_5
    }

    pub fn has_invoke_dynamic(&self) -> bool {
        self.major >= Self::MAJOR_7
    }

    pub fn has_lambdas(&self) -> bool {
        self.major >= Self::MAJOR_8
    }

    pub fn has_indy_string_concat(&self) -> bool {
        self.major >= Self::MAJOR_9
    }

    pub fn has_override(&self) -> bool {
        self.major >= Self::MAJOR_5
    }

    pub fn has_jsr(&self) -> bool {
        self.major <= Self::MAJOR_6
    }

    pub fn has_if_pattern_matching(&self) -> bool {
        self.major >= Self::MAJOR_16
    }

    pub fn has_switch_expressions(&self) -> bool {
        self.major >= Self::MAJOR_16
    }

    pub fn has_switch_pattern_match(&self) -> bool {
        self.preview_released(Self::MAJOR_17, Self::MAJOR_21)
    }

    pub fn has_record_pattern_matching(&self) -> bool {
        self.major >= Self::MAJOR_21
    }

    pub fn has_sealed_classes(&self) -> bool {
        self.preview_released(Self::MAJOR_15, Self::MAJOR_17)
    }

    pub fn has_local_enums_and_interfaces(&self) -> bool {
        self.major >= Self::MAJOR_16
    }

    pub fn has14_class_references(&self) -> bool {
        self.major <= Self::MAJOR_1_4
    }

    pub fn has_new_try_with_resources(&self) -> bool {
        self.major >= Self::MAJOR_11
    }

    pub fn predates_java(&self) -> bool {
        self.major <= Self::MAJOR_1_0_2 && self.minor <= 2
    }

    fn preview_from(&self, preview_start: i32) -> bool {
        self.major >= preview_start && self.minor == Self::PREVIEW
    }

    fn preview_released(&self, preview_start: i32, release_major: i32) -> bool {
        self.major >= release_major || self.preview_from(preview_start)
    }

    pub fn compare_to(&self, o: &BytecodeVersion) -> i32 {
        let cmp = self.major.cmp(&o.major) as i32;
        if cmp != 0 {
            return cmp;
        }
        self.minor.cmp(&o.minor) as i32
    }

    pub fn equals(&self, o: &BytecodeVersion) -> bool {
        self.major == o.major && self.minor == o.minor
    }

    pub fn hash_code(&self) -> i32 {
        (self.major << 16) | self.minor
    }
}

impl PartialOrd for BytecodeVersion {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BytecodeVersion {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.major
            .cmp(&other.major)
            .then(self.minor.cmp(&other.minor))
    }
}

impl JavaHash for BytecodeVersion {
    fn java_hash(&self) -> i32 {
        self.hash_code()
    }
}

impl JavaEq for BytecodeVersion {
    fn java_equals(&self, other: &Self) -> bool {
        self.equals(other)
    }
}
