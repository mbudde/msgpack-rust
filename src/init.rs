const FIXSTR_SIZE   : u8 = 0x1f;
const FIXARRAY_SIZE : u8 = 0x0f;
const FIXMAP_SIZE   : u8 = 0x0f;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Marker {
    PositiveFixnum(u8),
    NegativeFixnum(i8),
    Null,
    True,
    False,
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
    FixedString(u8),
    Str8,
    Str16,
    Str32,
    Bin8,
    Bin16,
    Bin32,
    FixedArray(u8),
    Array16,
    Array32,
    FixedMap(u8),
    Map16,
    Map32,
    FixExt1,
    FixExt2,
    FixExt4,
    FixExt8,
    FixExt16,
    Ext8,
    Ext16,
    Ext32,
    Reserved,
}

impl Marker {
    pub fn from_u8(n: u8) -> Marker {
        match n {
            0x00 ... 0x7f => Marker::PositiveFixnum(n),
            0xe0 ... 0xff => Marker::NegativeFixnum(n as i8),
            0x80 ... 0x8f => Marker::FixedMap(n & FIXMAP_SIZE),
            0x90 ... 0x9f => Marker::FixedArray(n & FIXARRAY_SIZE),
            0xa0 ... 0xbf => Marker::FixedString(n & FIXSTR_SIZE),
            0xc0 => Marker::Null,
            /// Marked in MessagePack spec as never used.
            0xc1 => Marker::Reserved,
            0xc2 => Marker::False,
            0xc3 => Marker::True,
            0xc4 => Marker::Bin8,
            0xc5 => Marker::Bin16,
            0xc6 => Marker::Bin32,
            0xc7 => Marker::Ext8,
            0xc8 => Marker::Ext16,
            0xc9 => Marker::Ext32,
            0xca => Marker::F32,
            0xcb => Marker::F64,
            0xcc => Marker::U8,
            0xcd => Marker::U16,
            0xce => Marker::U32,
            0xcf => Marker::U64,
            0xd0 => Marker::I8,
            0xd1 => Marker::I16,
            0xd2 => Marker::I32,
            0xd3 => Marker::I64,
            0xd4 => Marker::FixExt1,
            0xd5 => Marker::FixExt2,
            0xd6 => Marker::FixExt4,
            0xd7 => Marker::FixExt8,
            0xd8 => Marker::FixExt16,
            0xd9 => Marker::Str8,
            0xda => Marker::Str16,
            0xdb => Marker::Str32,
            0xdc => Marker::Array16,
            0xdd => Marker::Array32,
            0xde => Marker::Map16,
            0xdf => Marker::Map32,
            _ => Marker::Reserved,
        }
    }

    pub fn to_u8(&self) -> u8 {
        match *self {
            Marker::PositiveFixnum(val) => val,
            Marker::NegativeFixnum(val) => val as u8,

            Marker::Null                => 0xc0,

            Marker::True                => 0xc3,
            Marker::False               => 0xc2,

            Marker::U8                  => 0xcc,
            Marker::U16                 => 0xcd,
            Marker::U32                 => 0xce,
            Marker::U64                 => 0xcf,

            Marker::I8                  => 0xd0,
            Marker::I16                 => 0xd1,
            Marker::I32                 => 0xd2,
            Marker::I64                 => 0xd3,

            Marker::F32                 => 0xca,
            Marker::F64                 => 0xcb,

            Marker::FixedString(len)    => 0xa0 | (len & FIXSTR_SIZE),
            Marker::Str8                => 0xd9,
            Marker::Str16               => 0xda,
            Marker::Str32               => 0xdb,

            Marker::Bin8                => 0xc4,
            Marker::Bin16               => 0xc5,
            Marker::Bin32               => 0xc6,

            Marker::FixedArray(len)     => 0x90 | (len & FIXARRAY_SIZE),
            Marker::Array16             => 0xdc,
            Marker::Array32             => 0xdd,

            Marker::FixedMap(len)       => 0x80 | (len & FIXMAP_SIZE),
            Marker::Map16               => 0xde,
            Marker::Map32               => 0xdf,

            Marker::FixExt1             => 0xd4,
            Marker::FixExt2             => 0xd5,
            Marker::FixExt4             => 0xd6,
            Marker::FixExt8             => 0xd7,
            Marker::FixExt16            => 0xd8,
            Marker::Ext8                => 0xc7,
            Marker::Ext16               => 0xc8,
            Marker::Ext32               => 0xc9,

            Marker::Reserved            => 0xc1,
        }
    }
}
