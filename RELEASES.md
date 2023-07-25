Version 0.2.1 (2023-07-25)
==========================

- Remove derive(Debug) from generated Iter, IterMut, VacantEntry. This was forcing all elements to implement Debug. Users can implement Debug themselves trivially if needed.
- Add Display trait for generated keys. This will just be the same as `usize`.

Version 0.2.0 (2023-07-24)
==========================

- Implement Iter, IterMut, and Drain.

Version 0.1.1 (2023-07-19)
==========================

- Minor change to allow re-exporting wrapped_slab from other local crates.

Version 0.1.0 (2023-07-19)
==========================

- Initial version with only basic Slab methods implemented. Notably Iter and IterMut are missing, along with drain() and retain().