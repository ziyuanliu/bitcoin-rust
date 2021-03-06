#[derive(Debug, PartialEq, Display)]
pub enum Error {
    /// Low level database error
    #[display(fmt = "Database error: {}", _0)]
    DatabaseError(String),
    /// Invalid block
    #[display(fmt = "Cannot canonize block")]
    CannotCanonize,
    /// Unkown parent
    #[display(fmt = "Block parent is unknown")]
    UnknownParent,
    /// Ancient Fork
    #[display(fmt = "Fork is too long to proceed")]
    AncientFork,
}

impl From<Error> for String {
    fn from(e: Error) -> String {
        format!("{}", e)
    }
}
