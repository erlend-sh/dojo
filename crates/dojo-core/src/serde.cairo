use array::{ArrayTrait, SpanTrait};
use option::OptionTrait;
use serde::{Serde, serialize_array_helper, deserialize_array_helper};

impl SpanSerde<T,
impl TSerde: Serde<T>,
impl TCopy: Copy<T>,
impl TDrop: Drop<T>> of Serde<Span<T>> {
    fn serialize(self: @Span<T>, ref output: Array<felt252>) {
        (*self).len().serialize(ref output);
        serialize_array_helper(*self, ref output);
    }
    fn deserialize(ref serialized: Span<felt252>) -> Option<Span<T>> {
        let length = *serialized.pop_front()?;
        let mut arr = ArrayTrait::new();
        Option::Some(deserialize_array_helper(ref serialized, arr, length)?.span())
    }
}
