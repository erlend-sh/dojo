#[contract]
mod Database {
    use array::ArrayTrait;
    use array::SpanTrait;
    use traits::Into;

    use dojo_core::serde::SpanSerde;
    use dojo_core::storage::query::Query;
    use dojo_core::storage::query::QueryTrait;
    use dojo_core::storage::query::QueryIntoFelt252;
    use dojo_core::storage::kv::KeyValueStore;
    use dojo_core::storage::index::Index;

    use dojo_core::interfaces::IComponentLibraryDispatcher;
    use dojo_core::interfaces::IComponentDispatcherTrait;

    #[event]
    fn StoreSetRecord(table_id: felt252, keys: Span<felt252>, value: Span<felt252>) {}

    #[event]
    fn StoreSetField(table_id: felt252, keys: Span<felt252>, offset: u8, value: Span<felt252>) {}

    #[event]
    fn StoreDeleteRecord(tableId: felt252, keys: Span<felt252>) {}

    fn get(
        class_hash: starknet::ClassHash,
        table: felt252,
        query: Query,
        offset: u8,
        mut length: usize
    ) -> Option<Span<felt252>> {
        if length == 0_usize {
            length = IComponentLibraryDispatcher { class_hash: class_hash }.len()
        }

        let id = query.id();
        match Index::exists(table, id) {
            bool::False(()) => Option::None(()),
            bool::True(()) => Option::Some(KeyValueStore::get(table, id, offset, length))
        }
    }

    fn set(
        class_hash: starknet::ClassHash,
        table: felt252,
        query: Query,
        offset: u8,
        value: Span<felt252>
    ) {
        let keys = query.keys();
        let id = query.id();

        let length = IComponentLibraryDispatcher { class_hash: class_hash }.len();
        assert(value.len() <= length, 'Value too long');

        Index::create(table, id);
        KeyValueStore::set(table, id, offset, value);

        StoreSetRecord(table, keys, value);
        StoreSetField(table, keys, offset, value);
    }

    fn del(class_hash: starknet::ClassHash, table: felt252, query: Query) {
        Index::delete(table, query.into());
        // TODO: emit delete event
    }

    // returns a tuple of arrays, first contains the entity IDs
    // second the deserialized entities themselves
    fn all(
        class_hash: starknet::ClassHash,
        component: felt252,
        partition: felt252
    ) -> (Span<felt252>, Span<Span<felt252>>) {
        let table = {
            if partition == 0 {
                component
            } else {
                pedersen(component, partition)
            }
        };
        let all_ids = Index::query(table);
        let length = IComponentLibraryDispatcher { class_hash: class_hash }.len();

        let mut ids = all_ids.span();
        let mut entities: Array<Span<felt252>> = ArrayTrait::new();
        loop {
            match ids.pop_front() {
                Option::Some(id) => {
                    let value: Span<felt252> = KeyValueStore::get(table, *id, 0_u8, length);
                    entities.append(value);
                },
                Option::None(_) => {
                    break (all_ids.span(), entities.span());
                }
            };
        }
    }
}
