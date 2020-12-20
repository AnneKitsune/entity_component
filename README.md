# Entity Component
The entity-component part of a full ECS (Entity-Component-System).

# Usage
Add the following to you Cargo.toml file:
```
entity_component = "1.0.0"
```

Use it like so:
```rust
use game_clock::Time;
use std::time::Duration;
fn main() {
    let mut time = Time::default();
    time.set_fixed_time(Duration::from_secs_f64(1.0 / 20.0));

    let step = 1.0 / 60.0;
    for _ in 0..60 {
        time.advance_frame(Duration::from_secs_f64(step));
        { } // ...Run game logic, rendering, etc...
        while time.step_fixed_update() { // runs 20 times in a frame.
            { } // Run fixed frame logic (ie. physics)
        }
    }
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
