use crate::projection_body::ProjectionBody;

#[derive(Debug, PartialEq)]
pub struct Return<'a>(ProjectionBody<'a>);
