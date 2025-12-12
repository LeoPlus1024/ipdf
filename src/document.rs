use crate::bytes::{line_ending, literal_to_u64};
use crate::constants::TRAILER;
use crate::error::error_kind::{INVALID_CROSS_TABLE_ENTRY, INVALID_PDF_FILE, TRAILER_EXCEPT_A_DICT};
use crate::error::Result;
use crate::objects::{Entry, PDFObject, Xref};
use crate::parser::parse;
use crate::sequence::Sequence;
use crate::vpdf::PDFVersion;

/// Represent a PDF document
pub struct PDFDocument {
    /// Cross-reference table
    xrefs: Vec<Xref>,
    /// PDF version
    version: PDFVersion,
    /// PDF stream sequence
    sequence: Box<dyn Sequence>
}

impl PDFDocument {
    pub fn new(mut sequence: impl Sequence + 'static) -> Result<PDFDocument> {
        let version = parse_version(&mut sequence)?;
        let offset = find_xref_table_offset(&mut sequence)?;
        let xrefs = parse_xref(offset, &mut sequence)?;
        let pdf = PDFDocument {
            xrefs,
            version,
            sequence: Box::new(sequence),
        };
        Ok(pdf)
    }
    pub fn get_xref(&self) -> &Vec<Xref> {
        &self.xrefs
    }
    pub fn get_version(&self) -> &PDFVersion {
        &self.version
    }
}

fn parse_version(sequence: &mut impl Sequence) -> Result<PDFVersion> {
    let mut buf = [0u8; 1024];
    let n = sequence.read(&mut buf)?;
    if n < 8 {
        return Err(INVALID_PDF_FILE.into());
    }
    if buf.len() < 8
        || buf[0] != 37
        || buf[1] != 80
        || buf[2] != 68
        || buf[3] != 70
        || buf[4] != 45
    {
        return Err(INVALID_PDF_FILE.into());
    }
    let version = String::from_utf8(buf[5..8].to_vec())?;
    Ok(version.try_into()?)
}

fn parse_xref(offset: u64, sequence: &mut impl Sequence) -> Result<Vec<Xref>> {
    sequence.seek(offset)?;
    // Skip xref
    sequence.read_line()?;
    let xref_meta = sequence.read_line_str()?;
    let values = xref_meta.split_whitespace().collect::<Vec<&str>>();
    let obj_num = values[0].parse::<u64>()?;
    let length = values[1].parse::<u64>()?;
    let mut entries = Vec::<Entry>::with_capacity(length as usize);
    for i in 0..length {
        let line = sequence.read_line()?;
        if line.len() != 18 {
            return Err(INVALID_CROSS_TABLE_ENTRY.into());
        }
        let entry:Entry = String::from_utf8(line)?.try_into()?;
        entries.push(entry)
    }
    let xref = Xref {
        obj_num,
        length,
        entries,
    };
    let xrefs = vec![xref];
    let next_text = sequence.read_line_str()?;
    if next_text == TRAILER {
        let trailer = match parse(sequence)? {
            PDFObject::Dict(dict) => dict,
            _ =>return Err(TRAILER_EXCEPT_A_DICT.into())
        };
    }
    Ok(xrefs)
}

fn find_xref_table_offset(sequence: &mut impl Sequence) -> crate::error::Result<u64> {
    let size = sequence.size()?;
    let pos = if size > 1024 { size - 1024 } else { 0 };
    let mut buf = [0u8; 1024];
    sequence.seek(pos)?;
    let n = sequence.read(&mut buf)?;
    let mut list = Vec::<u8>::new();
    let mut index = 0;
    for i in (0..n).rev() {
        let b = buf[i];
        if b == b't' {
            break;
        }
        if b == b'%' {
            index = i;
        } else {
            if index != 0 && !line_ending(b) {
                list.push(b);
            }
        }
    }
    list.reverse();
    let byte_offset = literal_to_u64(&list);
    Ok(byte_offset)
}
