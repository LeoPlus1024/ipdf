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
    DirectObject(DirectObject),
    IndirectObject(IndirectObject),
    Stream(PDFStream),
}

pub struct PDFBool {
    value: bool,
}

#[derive(Eq, Hash, PartialEq)]
pub struct PDFNamed {
    pub(crate) name: String,
}

#[derive(PartialEq)]
pub enum Int {
    Signed(i64),
    Unsigned(u64),
}

#[derive(PartialEq)]
pub enum PDFNumber {
    Int(Int),
    Real(f64),
}

pub struct PDFString {}

pub struct PDFArray {
    elements: Vec<PDFObject>,
}

pub struct PDFDict {
    pub(crate) entries: HashMap<PDFNamed, PDFObject>,
}

pub struct DirectObject {
    obj_num: u32,
    gen_num: u16,
    value: Box<PDFObject>,
}

pub struct IndirectObject {
    obj_num: u32,
    gen_num: u16,
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