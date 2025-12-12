use std::collections::HashMap;
use crate::error::Error;
use crate::error::error_kind::{INVALID_CROSS_TABLE_ENTRY};

pub enum PDFObject{
    Bool(PDFBool),
    Number(PDFNumber),
    Named(PDFNamed),
    String(PDFString),
    Array(PDFArray),
    Dict(PDFDict),
    Null,
    DirectObject(PDFDirectObject),
    IndirectObject(PDFIndirectObject),
    Stream(PDFStream),
}

pub struct PDFBool {
    value: bool,
}

#[derive(Eq, Hash, PartialEq)]
pub struct PDFNamed {
    pub(crate) name: String,
}

#[derive(PartialEq,Clone)]
pub enum PDFNumber {
    Signed(i64),
    Unsigned(u64),
    Real(f64),
}


pub enum PDFString {
    Literal(Vec<u8>),
    Hex(Vec<u8>),
}

pub struct PDFArray {
    pub(crate) elements: Vec<PDFObject>,
}

pub struct PDFDict {
    pub(crate) entries: HashMap<PDFNamed, Option<PDFObject>>,
}

pub struct PDFDirectObject {
    pub(crate) obj_num: u64,
    pub(crate) gen_num: u64,
    pub(crate) metadata: Box<PDFObject>,
}

pub struct PDFIndirectObject {
    pub(crate) obj_num: u64,
    pub(crate) gen_num: u64,
}

pub struct PDFStream;


pub(crate) enum EntryState {
    Using(u64),
    Deleted(u64)
}


pub struct Entry {
    state: EntryState,
    /// The maximum generation number is 65535. Once that number is reached, that entry in the crossreference table will not be reused.
    gen_num: u16,
}
pub struct Xref {
    pub(crate) obj_num: u64,
    pub(crate) length: u64,
    pub(crate) entries: Vec<Entry>,
}

pub struct Trailer {
    pub(crate) metadata: PDFDict,
    pub(crate) byte_offset: u64,
}

impl TryFrom<String> for Entry {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let values = value.split_whitespace().collect::<Vec<&str>>();
        if values.len() != 3 {
            return Err(INVALID_CROSS_TABLE_ENTRY.into())
        }
        let value = values[0].parse::<u64>()?;
        let state = match values[2] {
            "n" => EntryState::Using(value),
            "f" => EntryState::Deleted(value),
            _ => return Err(INVALID_CROSS_TABLE_ENTRY.into())
        };
        let gen_num = values[1].parse::<u16>()?;
        Ok(Entry {
            state,
            gen_num,
        })
    }
}