# Entity Component
The entity-component part of a full ECS (Entity-Component-System).

# Usage
Add the following to you Cargo.toml file:
```
entity_component = "0.99.0"
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

### Focks Team Information
Maintainer: Jojolepro

Contact: jojolepro [at] jojolepro [dot] com

Commercial license available: yes

[Focks Team Website](https://jojolepro.com/focks)

### Licence
AGPL-3.0. You can buy commercial licenses [here](https://jojolepro.com/focks/).

See full license in LICENSE file.

Copyright (C) 2020 Jojolepro.
