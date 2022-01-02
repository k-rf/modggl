#[derive(Debug, PartialEq, Eq)]
pub enum EntryRelation {
    Equivalent,
    Less,
    Greater,
    LessOverlap,
    GreaterOverlap,
    LessInner,
    GreaterInner,
    LessOuter,
    GreaterOuter,
}
