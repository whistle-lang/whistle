#[derive(Debug, Clone, PartialEq)]
pub enum ErrorKind {
  ExpectedCommentInline,
  ExpectedCommentLine,

  ExpectedIdent,
  ExpectedKeyword,
  ExpectedBoolLit,
  ExpectedNoneLit,
  ExpectedFloatLit,
  ExpectedIntLit,
  
  ExpectedStringStartDelim,
  ExpectedStringInner,
  ExpectedStringEndDelim,
  ExpectedCharStartDelim,
  ExpectedCharInner,
  ExpectedCharEndDelim,

  ExpectedDig,
  ExpectedDec,
  ExpectedBin,
  ExpectedOct,
  ExpectedHex,
  ExpectedExp,
  ExpectedDecOrExp,

  ExpectedOperator,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LexerError {
    pub kind: ErrorKind,
    pub index: usize,
}

impl LexerError {
    pub fn new(kind: ErrorKind, index: usize) -> Self {
        Self {
            kind,
            index,
        }
    }
}

