use crate::simple::dynamic::DynProto;
use crate::simple::generic::GenProto;
use enum_flags::enum_flags;

/// Flags which control the bit stream format.
#[enum_flags]
#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum ProtoFlags {
    None = 0,
    OrderBe = 1,
    CopyGt7 = 2,
    Impl1Literal = 4,
    AlignStart = 8,
    FourOffsetTables = 16,
    ReuseOffset = 32,
}

/// Flags which control the bit stream traits.
#[enum_flags]
#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum TraitFlags {
    None = 0,
    LitSeq = 1,
    Len1Seq = 2,
    Len0123SeqMirrors = 4,
}

impl From<DynProto> for ProtoFlags {
    fn from(value: DynProto) -> Self {
        value.to_num().into()
    }
}

impl<const P: u8> From<GenProto<P>> for ProtoFlags {
    fn from(#[allow(unused_variables)] value: GenProto<P>) -> Self {
        P.into()
    }
}
