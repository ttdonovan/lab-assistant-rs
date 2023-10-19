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

3. General recommendations on how to organize code.

* Where to put the "fleet calculators"? See [https://www.npmjs.com/package/@staratlas/sage?activeTab=code]() and `@staratlas/sage/src/fleets.rs` and what should the api look like?

```
// pesudo example given ref to Fleet account (`&fleet`)
let fleet_calc = FleetCalculator::new();
let fleet_size = fleet_calc.get_fleet_size(&fleet);
```

or place all `calculate_*` functions in `staratlast::stage::fleet`?

* Thoughts about `staratlas::sage::starbase` see `lab-assistant-rs/src/staratals/sage/starbase.rs`.

4. Initialize Sage Labs game state and accounts given a `&Client` and player's `&Pubkey` to do execute operations:

    * move fleets
    * scan for SDU
    * mine resources
    * transport resources
    * load/unload cargo

    a. api `lab_assistant::init_sage_labs_game(&client, &player_pubkey)` returns struct `SagePlayerProfileGameState` that holds all required account information, ensure that assumptions in function are correct to "play" Sage Labs
    b. review the `SagePlayerProfileGameState` struct and identify missing Solana accounts and data required for above operations

## Credits

* Inspired by [Lab-Assistant](https://github.com/ImGroovin/Lab-Assistant).
