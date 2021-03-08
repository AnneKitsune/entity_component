# Entity Component
The entity-component part of a full ECS (Entity-Component-System).

# Why would you use this ECS library?

* Compatible with all platforms, including WASM!
* Public domain licensing: CC0
* Minimal amount of dependencies.
* Small code size.
* Stable, tested, benchmarked, 100% completed.

# Usage
Add the following to you Cargo.toml file:
```
entity_component = "1.0.0"
```

Use it like so:
```rust
use entity_component::*;

fn main() {
    // Creating components
    struct A(f32);
    struct B(f32);
    // Creating entity repository
    let mut entities = Entities::default();
    // Creating component storages
    let mut storage = Components::<A>::default();
    let mut storage2 = Components::<B>::default();
    // Create entities and add components
    for i in 0..10000 {
        let e = entities.create();
        if i % 5 == 0 {
            storage.insert(e, A(1.0));
        }
        if i % 6 == 0 {
            storage2.insert(e, B(1.0));
        }
    }
    // Join on all entities having both A and B.
    // We take a mutable reference to the A component and an immutable
    // reference to the B component.
    join!(&mut storage && &storage2)
        .for_each(|(s, s2)| s.unwrap().0 += s2.unwrap().0);

    // Same thing, but we also get the entities id that align with the
    // matched components.
    join!(&entities && &mut storage && &storage2)
        .for_each(|(_e, s, s2)| s.unwrap().0 += s2.unwrap().0);
}
```

### Maintainer Information

* Maintainer: Jojolepro
* Contact: jojolepro [at] jojolepro [dot] com
* Website: [jojolepro.com](https://jojolepro.com)
* Patreon: [patreon](https://patreon.com/jojolepro)

### Licence

CC0, public domain.

TLDR: You can do whatever you want with it. Have fun!

