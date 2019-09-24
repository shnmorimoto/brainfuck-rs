#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum TokenKind {
    Incr,
    Decr,
    Next,
    Prev,
    Read,
    Write,
    Open,
    Close,
}
