use crate::bytes::bytes_to_u64;
use crate::error::Result;
use crate::error::error_kind::{INVALID_PDF_FILE, TRAILER_NOT_FOUND};
use crate::vpdf::PDFVersion;
use crate::sequence::Sequence;

pub struct Entry {}
pub struct Xref {
    obj_num: u32,
    length: u32,
    entries: Vec<Entry>,
}
pub struct PDF {
    xref: Xref,
    version: PDFVersion,
}

pub fn parse(sequence:&mut impl Sequence) -> Result<PDF> {
    let version = parse_version(sequence)?;
    let xref = parse_xref(sequence)?;
    let pdf = PDF{
        xref: Xref{
            obj_num: 0,
            length: 0,
            entries: vec![],
        },
        version,
    };
    Ok(pdf)
}

fn parse_version(sequence:&mut impl Sequence) -> Result<PDFVersion> {
    let mut buf = [0u8; 1024];
    let n = sequence.read(&mut buf)?;
    if n < 8 {
        return Err(INVALID_PDF_FILE.into())
    }
    if buf.len() < 8
        || buf[0] != 37
        || buf[1] != 80
        || buf[2] != 68
        || buf[3] != 70
        || buf[4] != 45 {
        return Err(INVALID_PDF_FILE.into());
    }
    match String::from_utf8(buf[5..8].to_vec()) {
        Ok(v) => Ok(v.as_str().try_into()?),
        Err(_) => Err(INVALID_PDF_FILE.into()),
    }
}

fn parse_xref(sequence:&mut impl Sequence)->Result<Xref>{
    let offset = parse_trailer(sequence)?;
    Ok(Xref{
        obj_num: 0,
        length: 0,
        entries: vec![],
    })
}

fn parse_trailer(sequence: &mut impl Sequence) -> Result<u64> {
    let size = sequence.size()?;
    let pos = if size > 1024 {
        size - 1024
    } else {
        0
    };
    let mut buf = [0u8; 1024];
    sequence.seek(pos)?;
    let n = sequence.read(&mut buf)?;
    let mut list = Vec::<u8>::new();
    let mut index = 0;
    for i in (0..n).rev()  {
        // 't'
        let b = buf[i];
        if b == 102 {
            break;
        }
        // '%'
        if b == 37 {
            index = i;
        } else {
            if index != 0 && !line_ending(b) {
                list.push(b);
            }
        }
    }
    list.reverse();
    Ok(bytes_to_u64(&list))
}

#[inline]
fn line_ending(b: u8) -> bool {
    b == 10 || b == 13
}
