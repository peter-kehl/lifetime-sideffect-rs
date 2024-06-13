pub trait OutCollection<'out, T>
where
    T: 'out,
{
}

// COMPILES
pub fn caller_no_collection<'own: 'out, 'out, OwnType: 'own, OutType: 'out>(
    own_items: Vec<OwnType>,
) {
    called_no_collection::<'_, '_, OwnType, OutType>(&own_items);
}

pub fn called_no_collection<'own: 'out, 'out, OwnType: 'own, OutType: 'out>(
    _own_items: &'own Vec<OwnType>,
) {
}

// FAILS TO COMPILE
pub fn caller_with_collection<
    'own: 'out,
    'out,
    OwnType: 'own,
    OutType: 'out,
    OutCollectionType: OutCollection<'out, OutType> + 'own,
>(
    own_items: Vec<OwnType>,
) {
    called_with_collection::<'_, '_, OwnType, OutType, OutCollectionType>(&own_items);
    //                                                                    ^^^^^^^^^^
    //                                                                    |
    //                                                                    borrowed value does not live long enough
    // argument requires that `own_items` is borrowed for `'out`
}
// - `own_items` dropped here while still borrowed

pub fn called_with_collection<
    'own: 'out,
    'out,
    OwnType: 'own,
    OutType: 'out,
    OutCollectionType: OutCollection<'out, OutType> + 'own,
>(
    _own_items: &'own Vec<OwnType>,
) {
}
