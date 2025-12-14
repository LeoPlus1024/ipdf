use std::path::PathBuf;
use pdf_rs::document::PDFDocument;
use pdf_rs::error::Result;
#[test]
fn document() -> Result<()> {
    let document = PDFDocument::open(PathBuf::from("document/pdfreference1.0.pdf"))?;
    Ok(())
}
