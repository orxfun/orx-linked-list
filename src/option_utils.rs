#[inline(always)]
pub fn some_only_if<T>(some_condition: bool, value: Option<T>) -> Option<T> {
    if some_condition {
        value
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::some_only_if;

    #[test]
    fn some_only_if_true() {
        let none: Option<bool> = None;
        let result = some_only_if(true, none);
        assert_eq!(result, None);

        let some = Some(12);
        let result = some_only_if(true, some);
        assert_eq!(result, Some(12));
    }

    #[test]
    fn some_only_if_false() {
        let none: Option<bool> = None;
        let result = some_only_if(false, none);
        assert_eq!(result, None);

        let some = Some(12);
        let result = some_only_if(false, some);
        assert_eq!(result, None);
    }
}
