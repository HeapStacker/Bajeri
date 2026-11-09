//! Compact generational arena: store values and refer to them with stable ids.
//!
//! An [`Arena`] owns values of one type. [`Arena::insert`] returns an [`ArenaId`] that resolves
//! only while that value is still stored. [`Arena::remove`] hands the value back and frees the
//! slot. A later insert may reuse the slot, but the removed id does not resolve to the new value.
//!
//! Start with [`Arena::new`].

use std::any::Any;

/// Handle for one value stored in an [`Arena`].
///
/// The id stores two private facts. `index` is the slot inside the arena. `generation` is the
/// occupant number that was current when [`Arena::insert`] created this id. [`Arena::get`],
/// [`Arena::get_mut`], and [`Arena::remove`] succeed only when that slot still holds a value of
/// the same generation.
///
/// The same index can be reused after removal. The generation is what makes the old id fail and
/// the new id succeed. The id does not borrow the arena, so it can be copied and stored elsewhere.
/// It is not tied to the stored type: the same `ArenaId` can be passed to any [`Arena`]. It
/// resolves only when that arena's slot at `index` is still occupied by `generation`.
///
/// Construct an id only through [`Arena::insert`]. `ArenaId` is [`Copy`] and [`Clone`].
#[derive(Clone, Copy)]
pub struct ArenaId {
    index: usize,
    generation: u64,
}

impl ArenaId {
    /// Returns the slot index stored in this id.
    ///
    /// The index identifies a slot, not a live value. After removal, a new value can receive an
    /// id with the same index and a newer generation. Use the id with [`Arena::get`] to ask
    /// whether the original value is still there.
    ///
    /// This method does not panic and does not fail.
    ///
    /// # Examples
    ///
    /// ```
    /// use bajeri::Arena;
    ///
    /// let mut arena = Arena::new();
    /// let first = arena.insert("first");
    /// let second = arena.insert("second");
    ///
    /// assert_eq!(first.index(), 0);
    /// assert_eq!(second.index(), 1);
    /// ```
    pub fn index(&self) -> usize {
        self.index
    }
}

enum Entry<T> {
    Occupied {
        data: T,
        generation: u64,
    },
    Free {
        next_free: usize,
        generation: u64, // Ovdje čuvamo generaciju za sljedeći put
    },
}

/// Owns values of type `T` and hands out an [`ArenaId`] for each one.
///
/// The arena keeps every live value in a slot. [`insert`](Self::insert) either reuses a free slot
/// or appends a new one. [`remove`](Self::remove) returns the value and links that slot onto a
/// free list. [`len`](Self::len) counts live values only.
///
/// # Invariants
///
/// - `len` equals the number of occupied slots.
/// - A live id, used with `get`, returns the value that was inserted for that generation.
/// - An id that was removed does not resolve, including when its index is later reused.
///
/// Create an arena with [`Arena::new`] or [`Arena::with_capacity`].
pub struct Arena<T> {
    entries: Vec<Entry<T>>,
    free_head: Option<usize>,
    len: usize,
}

impl<T> Arena<T> {
    /// Creates an arena with no values and no reserved slot storage.
    ///
    /// The first inserts allocate storage as values are added.
    ///
    /// This method does not panic and does not fail.
    ///
    /// # Examples
    ///
    /// ```
    /// use bajeri::Arena;
    ///
    /// let arena = Arena::<u32>::new();
    /// assert!(arena.is_empty());
    /// assert_eq!(arena.len(), 0);
    /// ```
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            free_head: None,
            len: 0,
        }
    }

    /// Creates an empty arena that can accept `capacity` inserts before the slot storage grows.
    ///
    /// `capacity` reserves memory for slots. It does not insert values, so the arena is still empty.
    ///
    /// This method does not panic and does not fail.
    ///
    /// # Examples
    ///
    /// ```
    /// use bajeri::Arena;
    ///
    /// let mut arena = Arena::with_capacity(4);
    /// assert!(arena.is_empty());
    ///
    /// let id = arena.insert(7);
    /// assert_eq!(arena.get(id), Some(&7));
    /// ```
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            entries: Vec::with_capacity(capacity),
            free_head: None,
            len: 0,
        }
    }

    /// Inserts `data` and returns the id of that value.
    ///
    /// When a slot was freed by [`remove`](Self::remove), this reuses the most recently freed slot.
    /// The returned id uses the generation stored for the next occupant of that slot. When no slot
    /// is free, this appends a new slot and the id uses generation `0`.
    ///
    /// This method does not panic and does not fail.
    ///
    /// # Examples
    ///
    /// ```
    /// use bajeri::Arena;
    ///
    /// let mut arena = Arena::new();
    /// let id = arena.insert("alpha");
    /// assert_eq!(arena.get(id), Some(&"alpha"));
    /// assert_eq!(arena.len(), 1);
    /// ```
    pub fn insert(&mut self, data: T) -> ArenaId {
        let (index, generation) = if let Some(free_idx) = self.free_head {
            let (next_free, stored_gen) = match self.entries[free_idx] {
                Entry::Free {
                    next_free,
                    generation,
                } => (next_free, generation),
                _ => unreachable!(),
            };

            self.free_head = if next_free == usize::MAX {
                None
            } else {
                Some(next_free)
            };

            (free_idx, stored_gen)
        } else {
            let idx = self.entries.len();
            self.entries.push(Entry::Free {
                next_free: 0,
                generation: 0,
            });
            (idx, 0)
        };

        self.entries[index] = Entry::Occupied { data, generation };
        self.len += 1;

        ArenaId { index, generation }
    }

    /// Returns a shared reference to the value identified by `id`.
    ///
    /// `id` must come from an insert into this arena. The result is `Some` only when that slot is
    /// still occupied by the same generation.
    ///
    /// Returns `None` when the index is outside the arena, the slot is free, or the generation does
    /// not match because the value was removed. This method does not panic.
    ///
    /// # Examples
    ///
    /// ```
    /// use bajeri::Arena;
    ///
    /// let mut arena = Arena::new();
    /// let id = arena.insert(10);
    /// assert_eq!(arena.get(id), Some(&10));
    ///
    /// arena.remove(id);
    /// assert_eq!(arena.get(id), None);
    /// ```
    pub fn get(&self, id: ArenaId) -> Option<&T> {
        match &self.entries.get(id.index) {
            Some(Entry::Occupied { data, generation }) if *generation == id.generation => {
                Some(data)
            }
            _ => None,
        }
    }

    /// Returns a mutable reference to the value identified by `id`.
    ///
    /// The conditions are the same as for [`get`](Self::get). Changing the referenced value does
    /// not change the id. Returns `None` for an unknown, freed, or stale id. This method does not
    /// panic.
    ///
    /// # Examples
    ///
    /// ```
    /// use bajeri::Arena;
    ///
    /// let mut arena = Arena::new();
    /// let id = arena.insert(10);
    ///
    /// *arena.get_mut(id).unwrap() = 15;
    /// assert_eq!(arena.get(id), Some(&15));
    /// ```
    pub fn get_mut(&mut self, id: ArenaId) -> Option<&mut T> {
        match self.entries.get_mut(id.index) {
            Some(Entry::Occupied { data, generation }) if *generation == id.generation => {
                Some(data)
            }
            _ => None,
        }
    }

    /// Removes the value identified by `id` and returns it.
    ///
    /// On success the slot becomes free, its generation increases by one, and [`len`](Self::len)
    /// decreases by one. A later [`insert`](Self::insert) may reuse the index. `id` itself no
    /// longer resolves, so [`get`](Self::get) on it returns `None`.
    ///
    /// Returns `None` when `id` does not refer to a live value. In that case the arena is left
    /// unchanged. This method does not panic for a live, stale, or unknown id. Incrementing the
    /// generation panics in debug builds if that generation is already `u64::MAX`.
    ///
    /// # Examples
    ///
    /// ```
    /// use bajeri::Arena;
    ///
    /// let mut arena = Arena::new();
    /// let old = arena.insert("old");
    /// assert_eq!(arena.remove(old), Some("old"));
    ///
    /// let renewed = arena.insert("new");
    /// assert_eq!(renewed.index(), old.index());
    /// assert_eq!(arena.get(old), None);
    /// assert_eq!(arena.get(renewed), Some(&"new"));
    /// ```
    pub fn remove(&mut self, id: ArenaId) -> Option<T> {
        let entry = self.entries.get_mut(id.index)?;

        let current_gen = match entry {
            Entry::Occupied { generation, .. } => *generation,
            _ => return None,
        };

        if current_gen != id.generation {
            return None;
        }

        let next_gen = current_gen + 1;

        let old_entry = std::mem::replace(
            entry,
            Entry::Free {
                next_free: self.free_head.unwrap_or(usize::MAX),
                generation: next_gen,
            },
        );

        self.free_head = Some(id.index);
        self.len -= 1;

        match old_entry {
            Entry::Occupied { data, .. } => Some(data),
            _ => unreachable!(),
        }
    }

    /// Removes the value identified by `id` and reports whether it was present.
    ///
    /// This calls [`remove`](Self::remove) and drops the value instead of returning it. On success
    /// the slot becomes free, its generation increases by one, and [`len`](Self::len) decreases by
    /// one. A later [`insert`](Self::insert) may reuse the index. `id` itself no longer resolves.
    ///
    /// Returns `true` when a live value was removed. Returns `false` when `id` does not refer to a
    /// live value, and in that case the arena is left unchanged. This method does not panic for a
    /// live, stale, or unknown id. Incrementing the generation panics in debug builds if that
    /// generation is already `u64::MAX`, the same as [`remove`](Self::remove).
    ///
    /// # Examples
    ///
    /// ```
    /// use bajeri::Arena;
    ///
    /// let mut arena = Arena::new();
    /// let id = arena.insert("gone");
    /// assert!(arena.delete(id));
    /// assert!(!arena.delete(id));
    /// assert!(arena.is_empty());
    /// ```
    pub fn delete(&mut self, id: ArenaId) -> bool {
        self.remove(id).is_some()
    }

    /// Returns how many values are currently stored.
    ///
    /// Values that were removed are not counted. This method does not panic and does not fail.
    ///
    /// # Examples
    ///
    /// ```
    /// use bajeri::Arena;
    ///
    /// let mut arena = Arena::new();
    /// arena.insert(1);
    /// let id = arena.insert(2);
    /// arena.remove(id);
    /// assert_eq!(arena.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns whether the arena stores no values.
    ///
    /// This is true when [`len`](Self::len) is `0`. This method does not panic and does not fail.
    ///
    /// # Examples
    ///
    /// ```
    /// use bajeri::Arena;
    ///
    /// let mut arena = Arena::new();
    /// assert!(arena.is_empty());
    ///
    /// let id = arena.insert(1);
    /// assert!(!arena.is_empty());
    ///
    /// arena.remove(id);
    /// assert!(arena.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl<T> Default for Arena<T> {
    /// Creates an empty arena, the same value as [`Arena::new`].
    ///
    /// This method does not panic and does not fail.
    ///
    /// # Examples
    ///
    /// ```
    /// use bajeri::Arena;
    ///
    /// let arena = Arena::<u32>::default();
    /// assert!(arena.is_empty());
    /// ```
    fn default() -> Self {
        Self::new()
    }
}

/// Downcast helpers for an arena that stores type-erased values.
///
/// These methods exist only when the arena's value type is `Box<dyn Any>`. Each stored value is a
/// boxed [`Any`]. [`get_as`](Self::get_as), [`get_mut_as`](Self::get_mut_as), and
/// [`remove_as`](Self::remove_as) ask whether the value behind an id has concrete type `U`.
/// `U` must be `'static` because that is what [`Any`] can recover.
impl Arena<Box<dyn Any>> {
    /// Returns a shared reference to the value behind `id` when that value is a `U`.
    ///
    /// `id` must come from an insert into this arena. The result is `Some` only when that slot is
    /// still occupied by the same generation and the boxed value is a `U`.
    ///
    /// Returns `None` when the index is outside the arena, the slot is free, the generation does
    /// not match, or the stored value is not a `U`. A type mismatch leaves the value stored.
    /// This method does not panic.
    ///
    /// # Examples
    ///
    /// ```
    /// use bajeri::Arena;
    ///
    /// let mut arena = Arena::new();
    /// let id = arena.insert(Box::new(7u32) as Box<dyn std::any::Any>);
    /// assert_eq!(arena.get_as::<u32>(id), Some(&7));
    /// assert_eq!(arena.get_as::<i32>(id), None);
    /// assert_eq!(arena.get_as::<u32>(id), Some(&7));
    /// ```
    pub fn get_as<U: 'static>(&self, id: ArenaId) -> Option<&U> {
        let boxed_any = self.get(id)?; // vraća &Box<dyn Any>

        // Zahvaljujući deref koerciji, Box<dyn Any> se automatski pretvara u dyn Any
        boxed_any.downcast_ref::<U>()
    }

    /// Returns a mutable reference to the value behind `id` when that value is a `U`.
    ///
    /// The conditions are the same as for [`get_as`](Self::get_as). Changing the referenced value
    /// does not change the id and does not change the boxed type. Returns `None` for an unknown,
    /// freed, stale, or wrong-type id. A type mismatch leaves the value stored. This method does
    /// not panic.
    ///
    /// # Examples
    ///
    /// ```
    /// use bajeri::Arena;
    ///
    /// let mut arena = Arena::new();
    /// let id = arena.insert(Box::new(7u32) as Box<dyn std::any::Any>);
    /// *arena.get_mut_as::<u32>(id).unwrap() = 9;
    /// assert_eq!(arena.get_as::<u32>(id), Some(&9));
    /// ```
    pub fn get_mut_as<U: 'static>(&mut self, id: ArenaId) -> Option<&mut U> {
        let boxed_any = self.get_mut(id)?; // vraća &mut Box<dyn Any>

        boxed_any.downcast_mut::<U>()
    }

    /// Removes the value behind `id` and returns it boxed when that value is a `U`.
    ///
    /// On a matching type this is [`remove`](Self::remove) followed by a downcast. The slot
    /// becomes free, its generation increases by one, and [`len`](Self::len) decreases by one.
    /// `id` no longer resolves. The returned box owns the `U`.
    ///
    /// Returns `None` when `id` does not refer to a live value. In that case the arena is left
    /// unchanged.
    ///
    /// When the slot is live but the value is not a `U`, the value is still removed and then
    /// dropped. The slot is free, `id` no longer resolves, and the result is `None`. The original
    /// value is not put back.
    ///
    /// This method does not panic for a live, stale, unknown, or wrong-type id. Incrementing the
    /// generation panics in debug builds if that generation is already `u64::MAX`, the same as
    /// [`remove`](Self::remove).
    ///
    /// # Examples
    ///
    /// ```
    /// use bajeri::Arena;
    ///
    /// let mut arena = Arena::new();
    /// let id = arena.insert(Box::new(7u32) as Box<dyn std::any::Any>);
    /// let value = arena.remove_as::<u32>(id).unwrap();
    /// assert_eq!(*value, 7);
    /// assert!(arena.remove_as::<u32>(id).is_none());
    ///
    /// let other = arena.insert(Box::new("text") as Box<dyn std::any::Any>);
    /// assert!(arena.remove_as::<u32>(other).is_none());
    /// assert!(arena.is_empty());
    /// ```
    pub fn remove_as<U: 'static>(&mut self, id: ArenaId) -> Option<Box<U>> {
        let boxed_any = self.remove(id)?;

        boxed_any.downcast::<U>().ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn stale_id() -> ArenaId {
        ArenaId {
            index: 99,
            generation: 0,
        }
    }

    #[test]
    fn new_arena_is_empty() {
        let arena = Arena::<i32>::new();
        assert!(arena.is_empty());
        assert_eq!(arena.len(), 0);
    }

    #[test]
    fn default_matches_new() {
        let arena = Arena::<i32>::default();
        assert!(arena.is_empty());
        assert_eq!(arena.len(), 0);
    }

    #[test]
    fn with_capacity_starts_empty_and_stores_a_value() {
        let mut arena = Arena::with_capacity(4);
        assert!(arena.is_empty());
        let id = arena.insert(7);
        assert_eq!(arena.get(id), Some(&7));
        assert_eq!(arena.len(), 1);
    }

    #[test]
    fn insert_assigns_increasing_indexes() {
        let mut arena = Arena::new();
        let first = arena.insert("first");
        let second = arena.insert("second");
        assert_eq!(first.index(), 0);
        assert_eq!(second.index(), 1);
        assert_eq!(arena.get(first), Some(&"first"));
        assert_eq!(arena.get(second), Some(&"second"));
    }

    #[test]
    fn id_is_copy() {
        let mut arena = Arena::new();
        let id = arena.insert(1);
        let copied = id;
        assert_eq!(copied.index(), id.index());
        assert_eq!(arena.get(copied), Some(&1));
        assert_eq!(arena.get(id), Some(&1));
    }

    #[test]
    fn get_mut_updates_the_stored_value() {
        let mut arena = Arena::new();
        let id = arena.insert(10);
        *arena.get_mut(id).unwrap() = 15;
        assert_eq!(arena.get(id), Some(&15));
    }

    #[test]
    fn remove_returns_the_value_and_invalidates_the_id() {
        let mut arena = Arena::new();
        let id = arena.insert("gone");
        assert_eq!(arena.remove(id), Some("gone"));
        assert_eq!(arena.get(id), None);
        assert_eq!(arena.get_mut(id), None);
        assert!(arena.is_empty());
    }

    #[test]
    fn second_remove_of_a_freed_slot_returns_none() {
        let mut arena = Arena::new();
        let id = arena.insert(1);
        assert_eq!(arena.remove(id), Some(1));
        assert_eq!(arena.remove(id), None);
        assert_eq!(arena.len(), 0);
    }

    #[test]
    fn reused_slot_keeps_the_old_id_from_seeing_the_new_value() {
        let mut arena = Arena::new();
        let old = arena.insert("old");
        arena.remove(old);
        let renewed = arena.insert("new");

        assert_eq!(renewed.index(), old.index());
        assert_eq!(arena.get(old), None);
        assert_eq!(arena.get_mut(old), None);
        assert_eq!(arena.remove(old), None);
        assert_eq!(arena.get(renewed), Some(&"new"));
        assert_eq!(arena.len(), 1);
    }

    #[test]
    fn free_list_reuses_slots_from_the_most_recent_removal() {
        let mut arena = Arena::new();
        let a = arena.insert(10);
        let b = arena.insert(20);
        let c = arena.insert(30);
        assert_eq!(arena.len(), 3);

        assert_eq!(arena.remove(a), Some(10));
        assert_eq!(arena.remove(b), Some(20));
        assert_eq!(arena.remove(c), Some(30));
        assert!(arena.is_empty());

        let first = arena.insert(40);
        let second = arena.insert(50);
        let third = arena.insert(60);

        assert_eq!(first.index(), c.index());
        assert_eq!(second.index(), b.index());
        assert_eq!(third.index(), a.index());
        assert_eq!(arena.get(a), None);
        assert_eq!(arena.get(b), None);
        assert_eq!(arena.get(c), None);
        assert_eq!(arena.get(first), Some(&40));
        assert_eq!(arena.get(second), Some(&50));
        assert_eq!(arena.get(third), Some(&60));
        assert_eq!(arena.len(), 3);
    }

    #[test]
    fn delete_removes_a_live_value_and_rejects_a_second_call() {
        let mut arena = Arena::new();
        let id = arena.insert("gone");
        assert!(arena.delete(id));
        assert_eq!(arena.get(id), None);
        assert!(!arena.delete(id));
        assert!(arena.is_empty());
    }

    #[test]
    fn delete_of_an_unknown_id_leaves_the_arena_unchanged() {
        let mut arena = Arena::new();
        arena.insert(1);
        assert!(!arena.delete(stale_id()));
        assert_eq!(arena.len(), 1);
    }

    #[test]
    fn get_as_returns_the_concrete_value_and_keeps_a_mismatched_value() {
        let mut arena = Arena::new();
        let id = arena.insert(Box::new(7u32) as Box<dyn Any>);
        assert_eq!(arena.get_as::<u32>(id), Some(&7));
        assert_eq!(arena.get_as::<i32>(id), None);
        assert_eq!(arena.get_as::<u32>(id), Some(&7));
        assert_eq!(arena.get_as::<u32>(stale_id()), None);
    }

    #[test]
    fn get_mut_as_updates_the_concrete_value() {
        let mut arena = Arena::new();
        let id = arena.insert(Box::new(7u32) as Box<dyn Any>);
        *arena.get_mut_as::<u32>(id).unwrap() = 9;
        assert_eq!(arena.get_as::<u32>(id), Some(&9));
        assert!(arena.get_mut_as::<i32>(id).is_none());
        assert_eq!(arena.get_as::<u32>(id), Some(&9));
    }

    #[test]
    fn remove_as_returns_the_boxed_value_and_invalidates_the_id() {
        let mut arena = Arena::new();
        let id = arena.insert(Box::new(7u32) as Box<dyn Any>);
        let value = arena.remove_as::<u32>(id).unwrap();
        assert_eq!(*value, 7);
        assert!(arena.remove_as::<u32>(id).is_none());
        assert!(arena.is_empty());
    }

    #[test]
    fn remove_as_with_the_wrong_type_drops_the_stored_value() {
        let mut arena = Arena::new();
        let id = arena.insert(Box::new("text") as Box<dyn Any>);
        assert!(arena.remove_as::<u32>(id).is_none());
        assert!(arena.get(id).is_none());
        assert!(arena.is_empty());
    }

    #[test]
    fn unknown_id_does_not_resolve() {
        let mut arena = Arena::new();
        arena.insert(1);
        let id = stale_id();
        assert_eq!(arena.get(id), None);
        assert_eq!(arena.get_mut(id), None);
        assert_eq!(arena.remove(id), None);
        assert_eq!(arena.len(), 1);
    }
}
