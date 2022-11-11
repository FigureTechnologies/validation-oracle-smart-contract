use std::cmp::Ordering;

use crate::types::request::validation_request::ValidationRequestOrder;

type Comparator<T> = dyn Fn(&T, &T) -> Ordering;

pub const NO_VALIDATION_REQUEST_SORT: Option<&Comparator<ValidationRequestOrder>> =
    None::<&Comparator<ValidationRequestOrder>>;
