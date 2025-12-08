use std::collections::HashMap;

pub enum PDFObjectKind {
    Bool,
    Number,
    Named,
    String,
    Array,
    Dict,
}
pub trait PDFObject {
    /// Returns the bool value if the object is a bool.
    fn as_bool(&self) -> Option<&PDFBool> {
        None
    }

    /// Returns the number value if the object is a number.
    fn as_number(&self) -> Option<&PDFNumber> {
        None
    }

    /// Returns the named value if the object is a named.
    fn as_named(&self) -> Option<&PDFNamed> {
        None
    }

    /// Returns the string value if the object is a string.
    fn as_string(&self) -> Option<&PDFString> {
        None
    }

    /// Returns the array value if the object is an array.
    fn as_array(&self) -> Option<&PDFArray> {
        None
    }

    /// Returns the dict value if the object is a dict.
    fn as_dict(&self) -> Option<&PDFDict> {
        None
    }

    /// Returns the kind of the object.
    fn kind(&self) -> PDFObjectKind;
}

pub struct PDFBool {
    value: bool,
}

pub struct PDFNamed {
    name: String,
    value: Box<dyn PDFObject>,
}

pub enum Int {
    Signed(i64),
    Unsigned(u64),
}

pub enum PDFNumber {
    Int(Int),
    Real(f64),
}

pub struct PDFString {}

pub struct PDFArray {
    elements: Vec<Box<dyn PDFObject>>,
}

pub struct PDFDict {
    entries: HashMap<PDFNamed, Box<dyn PDFObject>>,
}

macro_rules! register_pdf_object {
    ($(($kind:ident,$tt:ty,$imp:ident)),+$(,)?) => {
        $(
        impl PDFObject for $tt {
            fn $imp(&self) -> Option<&$tt> {
                Some(self)
            }
               fn kind(&self) -> PDFObjectKind {
                    PDFObjectKind::$kind
                }
            }
        )+
    };
}

register_pdf_object!(
    (Bool, PDFBool, as_bool),
    (Number, PDFNumber, as_number),
    (Named, PDFNamed, as_named),
    (String, PDFString, as_string),
    (Array, PDFArray, as_array),
    (Dict, PDFDict, as_dict)
);
