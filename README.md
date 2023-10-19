# lab-assistant-rs

A framework for building automations within [Star Atlas](https://staratlas.com/) - SAGE Labs.

See [https://play.staratlas.com]() for more details.

## Rust Setup

See [rustup.rs](https://rustup.rs/) for toolchain installation.

## Development Setup

```
    git submodule init
    git submodule update
    # git submodule update --remote --merge
```

## Usage

Create your own scripts and place them in `lab-notebook/examples`, use `00_skratchpad.rs`
as a template for basic setup and usage.

```
    cargo run -p lab-notebook --example 00_skratchpad
```

## Requests For Help

1. See `lab-notebook/examples/00_skratchpad.rs` and `lab-assistant-rs/src/lib.rs`.

* Able to deserialize a `Fleet` account, but how to get "remaining data" for the fleet state? Using either `anchor_client` or `solana_client`.

2. See `lab-notebook/examples/00_skratchpad.rs`.

* What is wrong with `get_starbase_from_coords()`? Given `40, 30` would
expect Ustur CSS.

3. General recommendations on how to organize code.

* Where to put the "fleet calculators"? See [https://www.npmjs.com/package/@staratlas/sage?activeTab=code]() and `@staratlas/sage/src/fleets.rs` and what should the api look like?

```
// pesudo example given ref to Fleet account (`&fleet`)
let fleet_calc = FleetCalculator::new();
let fleet_size = fleet_calc.get_fleet_size(&fleet);
```

## Credits

* Inspired by [Lab-Assistant](https://github.com/ImGroovin/Lab-Assistant).
