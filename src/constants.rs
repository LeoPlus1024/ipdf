pub(crate) const TRAILER: &str = "trailer";
pub(crate) const LEFT_BRACKET: char = '<';
pub(crate) const RIGHT_BRACKET: char = '>';
pub(crate) const LEFT_PARENTHESIS: char = '(';
pub(crate) const RIGHT_PARENTHESIS: char = ')';
pub(crate) const DOUBLE_LEFT_BRACKET: &str = "<<";
pub(crate) const DOUBLE_RIGHT_BRACKET: &str = ">>";

pub(crate) const CR: char = '\r';
pub(crate) const LF: char = '\n';

pub(crate) const ESCAPE: char = '\\';

pub(crate) const SPACE: char = ' ';
pub(crate) const END_CHARS: [char; 7] = [
    LEFT_BRACKET,
    RIGHT_BRACKET,
    LEFT_PARENTHESIS,
    RIGHT_PARENTHESIS,
    CR,
    LF,
    ESCAPE,
];
