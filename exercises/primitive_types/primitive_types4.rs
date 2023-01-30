// primitive_types4.rs
// Get a slice out of Array a where the ??? is so that the test passes.
// Execute `rustlings hint primitive_types4` or use the `hint` watch subcommand for a hint.

#[test]
fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];

    let nice_slice = &a[1..4];

    // Following explanation from Copilot:

    // The type of nice_slice is &[i32]
    // which means it is a reference to a slice of i32s

    // A full explanation of &a[..]:
    // https://doc.rust-lang.org/std/primitive.slice.html#slicing-syntax
    // The &a means that we are taking a reference to the array.
    // The .. means that we are taking a range of the array.
    // We have to take a reference first because the range syntax
    // can only be used on things that implement the std::ops::RangeBounds trait.

    assert_eq!([2, 3, 4], nice_slice)
}
