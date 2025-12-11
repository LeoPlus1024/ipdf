use crate::constants::*;
use crate::error::error_kind::{EOF, EXCEPT_TOKEN, ILLEGAL_TOKEN, INVALID_NUMBER, STR_NOT_ENCODED, UNKNOWN_TOKEN};
use crate::error::{Error, Result};
use crate::objects::Int::Unsigned;
use crate::objects::{PDFDict, PDFNamed, PDFNumber, PDFObject, PDFString};
use crate::parser::Token::{Delimiter, Id, Number};
use crate::sequence::Sequence;
use std::collections::HashMap;
use std::ops::Range;

struct Tokenizer<'a> {
    buf: Vec<u8>,
    sequence: Box<&'a mut dyn Sequence>,
}

#[derive(PartialEq)]
enum Token {
    Id(String),
    Bool(bool),
    Keyword(String),
    Number(PDFNumber),
    Delimiter(String),
}

impl<'a> Tokenizer<'a> {
    fn new(sequence: &'a mut impl Sequence) -> Self {
        Self {
            sequence: Box::new(sequence),
            buf: Vec::with_capacity(1024),
        }
    }

    fn next_token(&mut self) -> Result<Token> {
        let option = self.next_chr()?;
        if option.is_none() {
            return Err(EOF.into());
        }
        let chr = option.unwrap();
        let token = match chr {
            LEFT_BRACKET => match self.next_chr_was('<') {
                true => Delimiter("<<".into()),
                false => Delimiter("<".into()),
            },
            RIGHT_BRACKET => match self.next_chr_was('>') {
                true => Delimiter(">>".into()),
                false => Delimiter(">".into()),
            },
            SPLASH |  LEFT_PARENTHESIS | RIGHT_PARENTHESIS => Delimiter(chr.into()),
            // CRLF
            CR | LF | WHITE_SPACE => self.next_token()?,
            chr => {
                // If the character is a digit, then we need to read the number
                if chr.is_digit(10) {
                    let value = self.num_deco(chr)?;
                    Number(value)
                }
                // Identifier
                else {
                    let range =self.loop_util(&END_CHARS,|_c| Ok(false))?;
                    let mut buf = self.buf.drain(range).collect::<Vec<u8>>();
                    buf.insert(0, chr as u8);
                    let id = String::from_utf8(buf)?;
                    Id(id)
                }
            }
        };
        Ok(token)
    }

    fn num_deco(&mut self, chr: char) -> Result<PDFNumber> {
        let range = self.loop_util(&END_CHARS, |c| {
            let is_digit = c.is_digit(10);
            if !is_digit {
                return Err(INVALID_NUMBER.into());
            }
            return Ok(false);
        })?;
        let mut bytes = self.buf.drain(range).collect::<Vec<u8>>();
        bytes.insert(0, chr as u8);
        let text = String::from_utf8(bytes)?;
        let number = if text.contains(".") {
            PDFNumber::Real(text.parse::<f64>()?)
        } else {
            PDFNumber::Int(Unsigned(text.parse::<u64>()?))
        };
        Ok(number)
    }

    fn loop_util<F>(&mut self, end_chars: &[char], mut func: F) -> Result<Range<usize>>
    where
        F: FnMut(char) -> Result<bool>,
    {
        let mut index = 0usize;
        let buf = &mut self.buf;
        'ext: loop {
            if buf.is_empty() {
                let n = self.sequence.read(buf)?;
                if n == 0 {
                    return Err(EOF.into());
                }
            }
            let len = buf.len();
            for i in index..len {
                let chr = char::from(buf[i]);
                if end_chars.contains(&chr) || func(chr)? {
                    index = i;
                    break 'ext;
                }
            }
        }
        Ok(0..index)
    }

    fn next_chr(&mut self) -> Result<Option<char>> {
        let option = match self.next_chr0(|_| true)? {
            None => None,
            Some((_, chr)) => Some(chr),
        };
        Ok(option)
    }

    fn next_chr_was(&mut self, chr: char) -> bool {
        match self.next_chr0(|c| c == chr) {
            Ok(Some((equal, _))) => equal,
            _ => false,
        }
    }

    /// Read next byte
    fn next_chr0<F>(&mut self, func: F) -> Result<Option<(bool, char)>>
    where
        F: Fn(char) -> bool,
    {
        let buf = &mut self.buf;
        let mut bytes = [0u8; 1024];
        if buf.is_empty() {
            let n = self.sequence.read(&mut bytes)?;
            if n == 0 {
                return Ok(None);
            }
            buf.extend_from_slice(&bytes[0..n]);
        }
        let b = buf[0];
        let chr = char::from(b);
        let equal = func(chr);
        if equal {
            buf.remove(0);
        }
        Ok(Some((equal, chr)))
    }
}

pub(crate) fn parse(sequence: &mut impl Sequence) -> Result<PDFObject> {
    let mut tokenizer = Tokenizer::new(sequence);
    let token = tokenizer.next_token()?;
    let object = parser0(&mut tokenizer, token)?;
    Ok(object)
}

fn parser0(tokenizer: &mut Tokenizer, token: Token) -> Result<PDFObject> {
    match token {
        Delimiter(delimiter)=>{
            match delimiter.as_str() {
                "<<" => parse_dict(tokenizer),
                "/" => parse_named(tokenizer),
                "<" | "(" => parse_string(tokenizer, delimiter == "("),
                &_ => todo!()
            }
        }
        _ => panic!("")
    }
}


fn parse_dict(mut tokenizer: &mut Tokenizer) -> Result<PDFObject> {
    let mut entries =HashMap::<PDFNamed, Option<PDFObject>>::new();
    loop {
        let token = tokenizer.next_token()?;
        if let Delimiter(ref delimiter) =  token {
            if delimiter == DOUBLE_RIGHT_BRACKET {
                return Ok(PDFObject::Dict(PDFDict { entries }));
            }
        }
        let object = parser0(&mut tokenizer, token)?;
        if let PDFObject::Named(named) = object {
            let token = tokenizer.next_token()?;
            if let Delimiter(ref delimiter) = token {
                let dict_close = *delimiter == DOUBLE_RIGHT_BRACKET;
                let is_named = *delimiter == String::from(SPLASH);
                if is_named || dict_close {
                    entries.insert(named, None);
                    if dict_close {
                        return Ok(PDFObject::Dict(PDFDict { entries }));
                    }
                    continue;
                }
            }
            let value = parser0(&mut tokenizer, token)?;
            entries.insert(named, Some(value));
        }
        return Err(Error::new(UNKNOWN_TOKEN, "Except a named token.".into()));

    }
}

fn parse_named(tokenizer: &mut Tokenizer) -> Result<PDFObject> {
    let token = tokenizer.next_token()?;
    if let Id(name) = token {
        return Ok(PDFObject::Named(PDFNamed { name }));
    }
    Err(Error::new(EXCEPT_TOKEN,"Except a identifier token.".to_string()))
}

fn parse_string(tokenizer: &mut Tokenizer, post_script: bool) -> Result<PDFObject> {
    let end_chr = if post_script { ')' } else { '>' };
    let mut is_escape = true;
    let result = tokenizer.loop_util(&[], |chr| {
        is_escape = (chr == ESCAPE) && !is_escape;
        Ok(is_escape || chr == end_chr)
    });
    match result {
        Ok(range) => {
            let buf = tokenizer.buf.drain(range).collect::<Vec<u8>>();
            let pdf_str = if post_script {
                PDFString::Hex(buf)
            } else {
                PDFString::Literal(buf)
            };
            Ok(PDFObject::String(pdf_str))
        }
        Err(_e) => Err(STR_NOT_ENCODED.into())
    }
}