#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    if first_list == second_list {
        return Comparison::Equal;
    }
    if is_a_superlist_of_b(first_list, second_list) {
        return Comparison::Superlist;
    }
    if is_a_superlist_of_b(second_list, first_list) {
        return Comparison::Sublist;
    }
    Comparison::Unequal
}

fn is_a_superlist_of_b<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    b.is_empty() || a.windows(b.len()).any(|a_slice| a_slice == b)
}
