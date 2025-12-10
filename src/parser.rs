use crate::objects::{PDFNull, PDFObject};
use crate::sequence::Sequence;
use crate::error::Result;
use crate::objects::PDFObjectKind::Dict;
use crate::parser::Token::{Delimiter, Illegal, _None};

struct Tokenizer<'a> {
    buf: Vec<u8>,
    sequence: Box<&'a mut dyn Sequence>,
}

enum Token {
    Bool(bool),
    Keyword(String),
    Number(String),
    Delimiter(char),
    Illegal,
    _None
}

impl<'a> Tokenizer<'a> {
    fn new(sequence: &'a mut impl Sequence) -> Self {
        Self {
            sequence: Box::new(sequence),
            buf: Vec::with_capacity(1024),
        }
    }

    fn next_token(&mut self) -> Result<Token> {
        let option = self.next_byte()?;
        if option.is_none() {
            return Ok(_None);
        }
        let token = match option.unwrap() {
            b'<' => self.parse_dict_or_ps_str()?,
            _ => Illegal
        };
        Ok(token)
    }

    fn parse_dict_or_ps_str(&mut self)->Result<Token> {
        Ok(Delimiter('<'))
    }

    /// Read next byte
    fn next_byte(&mut self) -> Result<Option<u8>> {
        let buf = &mut self.buf;
        if buf.is_empty() {
            let n = self.sequence.read(buf)?;
            if n == 0 {
                return Ok(None);
            }
        }
        Ok(Some(buf.remove(0)))
    }
}

pub(crate) fn parse(sequence: &mut impl Sequence) -> Result<impl PDFObject> {
    let tokenizer = Tokenizer::new(sequence);

    Ok(PDFNull)
}
