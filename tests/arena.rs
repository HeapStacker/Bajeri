use bajeri::Arena;

#[test]
fn public_api_inserts_reads_updates_and_removes() {
    let mut arena = Arena::new();
    assert!(arena.is_empty());

    let wolf = arena.insert("wolf");
    let eagle = arena.insert("eagle");
    assert_eq!(wolf.index(), 0);
    assert_eq!(eagle.index(), 1);
    assert_eq!(arena.len(), 2);
    assert_eq!(arena.get(wolf), Some(&"wolf"));

    *arena.get_mut(eagle).unwrap() = "hawk";
    assert_eq!(arena.remove(eagle), Some("hawk"));
    assert_eq!(arena.get(eagle), None);
    assert_eq!(arena.len(), 1);

    let bear = arena.insert("bear");
    assert_eq!(bear.index(), eagle.index());
    assert_eq!(arena.get(eagle), None);
    assert_eq!(arena.get(bear), Some(&"bear"));
    assert!(!arena.is_empty());
}

#[test]
fn an_id_from_one_arena_is_accepted_by_another() {
    let mut names = Arena::new();
    let mut numbers = Arena::new();
    let name = names.insert("wolf");
    numbers.insert(7);

    assert_eq!(names.get(name), Some(&"wolf"));
    assert_eq!(numbers.get(name), Some(&7));
}

#[test]
fn with_capacity_and_a_second_remove_use_the_public_api() {
    let mut arena = Arena::with_capacity(2);
    let id = arena.insert(5);
    assert_eq!(arena.get(id), Some(&5));
    assert_eq!(arena.remove(id), Some(5));
    assert_eq!(arena.remove(id), None);
    assert!(arena.is_empty());
}
