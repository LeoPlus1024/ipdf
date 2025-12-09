use crate::error::Result;
use crate::objects::Xref;
use crate::parser::{parse_version, parse_xref};
use crate::sequence::Sequence;
use crate::vpdf::PDFVersion;

/// Represent a PDF document
pub struct PDFDocument {
    /// Cross-reference table
    xref: Xref,
    /// PDF version
    version: PDFVersion,
    /// PDF stream sequence
    sequence: Box<dyn Sequence>
}

impl PDFDocument {
    pub fn new(mut sequence: impl Sequence + 'static) -> Result<PDFDocument> {
        let version = parse_version(&mut sequence)?;
        let xref = parse_xref(&mut sequence)?;
        let pdf = PDFDocument {
            xref,
            version,
            sequence: Box::new(sequence),
        };
        Ok(pdf)
    }

    pub fn get_xref(&self) -> &Xref {
        &self.xref
    }
    pub fn get_version(&self) -> &PDFVersion {
        &self.version
    }
}
