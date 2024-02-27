use crate::{
    list::List,
    variants::{doubly::Doubly, ends::ListEnds, list_variant::ListVariant, singly::Singly},
};

impl<'a, V, T> List<'a, V, T>
where
    V: ListVariant<'a, T>,
    V::Ends: ListEnds<'a, V, T>,
{
    #[cfg(test)]
    fn validate_next(&self) {
        use orx_selfref_col::NodeRefs;
        let mut count = 0;

        let mut next = self.col.ends().front();
        while let Some(current) = next {
            count += 1;
            next = *current.next().get();
        }

        assert_eq!(count, self.len());
    }
}

impl<'a, T> List<'a, Singly, T> {
    #[cfg(test)]
    pub(crate) fn validate_list(&self) {
        self.validate_next();
    }
}

impl<'a, T> List<'a, Doubly, T> {
    #[cfg(test)]
    pub(crate) fn validate_list(&self) {
        self.validate_next();
        self.validate_prev();
    }

    #[cfg(test)]
    fn validate_prev(&self) {
        use orx_selfref_col::NodeRefs;
        let mut count = 0;

        let mut prev = self.col.ends().back();
        while let Some(current) = prev {
            count += 1;
            prev = *current.prev().get();
        }

        assert_eq!(count, self.len(), "count & len mismatch");
    }
}
