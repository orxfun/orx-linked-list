use super::List;
use crate::variant::Singly;
use orx_selfref_col::MemoryPolicy;

impl<T, M> List<Singly<T>, M> where M: MemoryPolicy<Singly<T>> {}
