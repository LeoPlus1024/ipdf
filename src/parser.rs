use crate::constants::*;
use crate::error::error_kind::{EOF, EXCEPT_TOKEN, INVALID_NUMBER, UNKNOWN_TOKEN};
use crate::error::{Error, Result};
use crate::objects::Int::Unsigned;
use crate::objects::{PDFDict, PDFNamed, PDFNumber, PDFObject, PDFStream};
use crate::parser::Token::{Delimiter, Number};
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

impl Token {
    fn except_delimiter(&self, chr: char)->Result<()> {
        let tmp = chr.to_string();
        match self {
            Delimiter(delimiter) => {
                if *delimiter == tmp {
                    return Ok(());
                }
                Err(Error::new(EXCEPT_TOKEN, format!("Expected {} but got {}", tmp, delimiter)))
            },
            _ => Err(Error::new(UNKNOWN_TOKEN, format!("Excepted a {}",tmp))),
        }
    }
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
            '<' => match self.next_chr_was('<') {
                true => Delimiter("<<".into()),
                false => Delimiter(">>".into()),
            },
            '>' => match self.next_chr_was('>') {
                true => Delimiter(">>".into()),
                false => Delimiter(">".into()),
            },
            '/' => Delimiter("/".into()),
            '(' | ')' => Delimiter(chr.into()),
            '\r' | '\n' => self.next_token()?,
            chr => {
                // If the character is a digit, then we need to read the number
                if chr.is_digit(10) {
                    let value = self.num_deco(chr)?;
                    Number(value)
                } else {
                    return Err(UNKNOWN_TOKEN.into())
                }
            }
        };
        Ok(token)
    }

    fn num_deco(&mut self, chr: char) -> Result<PDFNumber> {
        let range = self.loop_util(|c| {
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

    fn loop_util<F>(&mut self, func: F) -> Result<Range<usize>>
    where
        F: Fn(char) -> Result<bool>,
    {
        let mut index = 0usize;
        let buf = &mut self.buf;
        let end_chars = &END_CHARS;
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
                &_ => todo!()
            }
        }
        _ => panic!("")
    }
}


fn parse_dict(mut tokenizer: &mut Tokenizer) -> Result<PDFObject> {
    let mut entries =HashMap::<PDFNamed, PDFObject>::new();
    loop {
        let token = tokenizer.next_token()?;
        if let Delimiter(ref delimiter) =  token {
            if delimiter == DOUBLE_RIGHT_BRACKET {
                return Ok(PDFObject::Dict(PDFDict { entries }));
            }
        }
        match parser0(&mut tokenizer, token)? {
            PDFObject::Named(name) => {
                let token = tokenizer.next_token()?;
                let value = parser0(&mut tokenizer, token)?;
                entries.insert(name, value);
            }
            _ => return Err(Error::new(UNKNOWN_TOKEN, "Except a named token.".into()))
        }
    }
}

fn parse_named(tokenizer: &mut Tokenizer)->Result<PDFObject>{
    let token = tokenizer.next_token()?;
    if let Token::Id(name) = token {
        return Ok(PDFObject::Named(PDFNamed { name }));
    }
    Err(Error::new(EXCEPT_TOKEN,"Except a identifier token.".to_string()))
}
