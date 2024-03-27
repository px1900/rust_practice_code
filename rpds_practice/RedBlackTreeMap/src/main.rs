use std::ops::Range;
use rpds::RedBlackTreeMap;

fn main() {
    let map_en = RedBlackTreeMap::new()
        .insert(0, "zero")
        .insert(1, "one");

    assert_eq!(map_en.get(&1), Some(&"one"));

    let map_pt = map_en
        .insert(1, "um")
        .insert(2, "dois");

    assert_eq!(map_pt.get(&2), Some(&"dois"));

    let map_pt_binary = map_pt.remove(&2);

    assert_eq!(map_pt_binary.get(&2), None);

    assert_eq!(map_pt_binary.first(), Some((&0, &"zero")));


    dbg!(map_en);
    dbg!(map_pt);
    dbg!(map_pt_binary);

    let map_fruit = RedBlackTreeMap::new()
        .insert("a", 1)
        .insert("b", 2)
        .insert("c", 3)
        .insert("d", 4)
        .insert("e", 5);

    dbg!(&map_fruit);

    let range_key = Range { start: "b", end: "d" };
    let v = map_fruit.range(range_key).collect::<Vec<_>>();
    dbg!(v);

}