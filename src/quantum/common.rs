pub trait Equivalency {
    fn are_equivalent(&self, other: &Self) -> bool;
}
