use kinded::Kinded;
use proc_macro2::Span;

use super::models::{NumericBoundValidator, SpannedItem};

pub fn validate_duplicates<T>(
    items: &[SpannedItem<T>],
    build_error_msg: impl Fn(<T as Kinded>::Kind) -> String,
) -> Result<(), syn::Error>
where
    T: Kinded,
{
    if let Some((item1, item2)) = detect_items_of_same_kind(items) {
        assert_eq!(item1.kind(), item2.kind());
        let kind = item1.kind();
        let msg = build_error_msg(kind);
        let span = join_spans_or_last(item1.span(), item2.span());
        let err = syn::Error::new(span, msg);
        return Err(err);
    }
    Ok(())
}

fn detect_items_of_same_kind<T: Kinded>(items: &[T]) -> Option<(&T, &T)> {
    // Note: this has O(n^2) complexity, but it's not a problem, because size of collection is < 10.
    for (i1, item1) in items.iter().enumerate() {
        for (i2, item2) in items.iter().enumerate() {
            if i1 != i2 && item1.kind() == item2.kind() {
                return Some((item1, item2));
            }
        }
    }
    None
}

fn join_spans_or_last(span1: Span, span2: Span) -> Span {
    span1.join(span2).unwrap_or(span2)
}

macro_rules! find_bound_variant {
    ($validators:ident, $method:ident) => {
        $validators
            .iter()
            .flat_map(|validator| {
                if let Some(value) = validator.item.$method() {
                    Some(SpannedItem::new(value, validator.span()))
                } else {
                    None
                }
            })
            .next()
    };
}

pub fn validate_numeric_bounds<V, T>(validators: &[SpannedItem<V>]) -> Result<(), syn::Error>
where
    V: NumericBoundValidator<T>,
    T: Clone + PartialOrd,
{
    let maybe_greater = find_bound_variant!(validators, greater);
    let maybe_greater_or_equal = find_bound_variant!(validators, greater_or_equal);
    let maybe_less = find_bound_variant!(validators, less);
    let maybe_less_or_equal = find_bound_variant!(validators, less_or_equal);

    // greater VS greater_or_equal
    //
    if let (Some(_), Some(ge)) = (maybe_greater.clone(), maybe_greater_or_equal.clone()) {
        let msg = "The lower bound can be specified with EITHER `greater` OR `greater_or_equal`, but not both.";
        let err = syn::Error::new(ge.span(), msg);
        return Err(err);
    }
    // less VS less_or_equal
    //
    if let (Some(_), Some(le)) = (maybe_less.clone(), maybe_less_or_equal.clone()) {
        let msg =
            "The upper bound can be specified with EITHER `less` OR `less_or_equal`, but not both.";
        let err = syn::Error::new(le.span(), msg);
        return Err(err);
    }

    let maybe_lower_bound = maybe_greater.or(maybe_greater_or_equal);
    let maybe_upper_bound = maybe_less.or(maybe_less_or_equal);

    // less_or_equal VS greater_or_equal
    //
    if let (Some(lower), Some(upper)) = (maybe_lower_bound, maybe_upper_bound) {
        if lower.item > upper.item {
            let msg = "The lower bound (`greater` or `greater_or_equal`) cannot be greater than the upper bound (`less or `less_or_equal`).\nSometimes we all need a little break.";
            let err = syn::Error::new(upper.span(), msg);
            return Err(err);
        }
    }

    Ok(())
}
