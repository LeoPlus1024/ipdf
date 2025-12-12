use crate::constants::*;
use crate::error::error_kind::{EXCEPT_TOKEN, STR_NOT_ENCODED, UNKNOWN_TOKEN};
use crate::error::{Error, Result};
use crate::objects::{PDFArray, PDFDict, PDFDirectObject, PDFIndirectObject, PDFNamed, PDFNumber, PDFObject, PDFString};
use crate::sequence::Sequence;
use std::collections::HashMap;
use crate::tokenizer::{Token, Tokenizer};
use crate::tokenizer::Token::{Delimiter, Id, Number};

pub(crate) fn parse(sequence: &mut impl Sequence) -> Result<PDFObject> {
    let mut tokenizer = Tokenizer::new(sequence);
    let token = tokenizer.next_token()?;
    let object = parser0(&mut tokenizer, token)?;
    Ok(object)
}

fn parser0(tokenizer: &mut Tokenizer, token: Token) -> Result<PDFObject> {
    match token {
        Delimiter(delimiter) => match delimiter.as_str() {
            DOUBLE_LEFT_BRACKET => parse_dict(tokenizer),
            "[" => parse_array(tokenizer),
            "/" => parse_named(tokenizer),
            "<" | "(" => parse_string(tokenizer, delimiter == "("),
            &_ => todo!(),
        },
        Number(number) => match number {
            PDFNumber::Unsigned(value) => {
                let is_num = tokenizer.check_next_token(|token| Ok(token.is_unsigned()))?;
                if !is_num {
                    return Ok(PDFObject::Number(number));
                }
                let is_obj = tokenizer.check_next_token(|token| Ok(token.id_is(R) || token.id_is(OBJ)))?;
                if is_obj {
                    return parse_obj(tokenizer, Some(value))
                }
                Ok(PDFObject::Number(number))
            }
            _ => Ok(PDFObject::Number(number))
        },
        _ => panic!(""),
    }
}

fn parse_obj(tokenizer: &mut Tokenizer, option: Option<u64>) -> Result<PDFObject> {
    let obj_num = match option {
        Some(num) => num,
        None => tokenizer.next_token()?.unsigned_num()?
    };
    let obj_gen_token = tokenizer.next_token()?.except(|token| token.is_unsigned())?;
    let type_token = tokenizer.next_token()?.except(|token| token.is_id())?;
    let gen_num = obj_gen_token.unsigned_num()?;
    if let Id(ref id) = type_token {
        let object = match id.as_str() {
            OBJ => {
                let metadata = parse_dict(tokenizer)?;
                let object = PDFDirectObject {
                    obj_num,
                    gen_num,
                    metadata: Box::new(metadata),
                };
                PDFObject::DirectObject(object)
            },
            _ => {
                let object = PDFIndirectObject {
                    obj_num,
                    gen_num,
                };
                PDFObject::IndirectObject(object)
            }
        };
        return Ok(object)
    }
    Err(Error::new(EXCEPT_TOKEN, "Except a token with R or obj".to_string()))

}
fn parse_dict(mut tokenizer: &mut Tokenizer) -> Result<PDFObject> {
    let mut entries = HashMap::<PDFNamed, Option<PDFObject>>::new();
    loop {
        let token = tokenizer.next_token()?;
        if let Delimiter(ref delimiter) = token {
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
        } else {
            return Err(Error::new(UNKNOWN_TOKEN, "Except a named token.".into()));
        }
    }
}

fn parse_named(tokenizer: &mut Tokenizer) -> Result<PDFObject> {
    let token = tokenizer.next_token()?;
    if let Id(name) = token {
        return Ok(PDFObject::Named(PDFNamed { name }));
    }
    Err(Error::new(
        EXCEPT_TOKEN,
        "Except a identifier token.".to_string(),
    ))
}

fn parse_array(tokenizer: &mut Tokenizer) -> Result<PDFObject> {
    let mut elements = Vec::<PDFObject>::new();
    loop {
        let token = tokenizer.next_token()?;
        if let Delimiter(ref delimiter) = token {
            if delimiter == "]" {
                return Ok(PDFObject::Array(PDFArray { elements }));
            }
        }
        let object = parser0(tokenizer, token)?;
        elements.push(object);

    }
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
            // Remove '>' or ')'
            tokenizer.buf.remove(0);
            Ok(PDFObject::String(pdf_str))
        }
        Err(_e) => Err(STR_NOT_ENCODED.into()),
    }
}
