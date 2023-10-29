# lab-assistant-rs (*HOLD*)

**_Note_** current development is on hold...

## Rust Setup

For toolchain installation, see [rustup.rs](https://rustup.rs/).

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

1. See `lab-notebook/examples/00_skratchpad.rs` (dbg! and comments) and `lab-assistant-rs/src/lib.rs`.

* Able to deserialize a `Fleet` account, but how to get "remaining data" (sorta works) for the fleet state?
* Sorta working but very confused on the bytes return from RPC solona `get_account_data()`
* See `lab-assistant-rs/src/staratlas/sage/fleet.rs` and specfically `get_fleet_state()`

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
    * mine resources (current exploration, the goal to start/stop mining for a fleet)
    * transport resources
    * load/unload cargo

    a. api `lab_assistant::init_sage_labs_game(&client, &player_pubkey)` returns struct `SagePlayerProfileGameState` that holds all required account information
    b. ensure that assumptions outline in function `init_sage_labs_game()` are correct to "play" Sage Labs
    c. review the `SagePlayerProfileGameState` struct and identify missing Solana accounts and data required for above operations

5. Struggle with understanding how to use `anchor-client` program `request()` "RequestBuilder" and instructions without idl generated code...
    * alternatively could all the instructions be built via `solana_program` manually?

6. Feedback on API/DSL code organization and usage (current thought):

    * entity/struct to hold "state" (on-chain/off-chain) information see `LabAssistant` (and `SagePlayerProfileGameState`)
    * use "handlers" to eveluate "fleets" determine what can/cannot be done by issueing a "command"
    * use "commands" to hold information about what should be done (dsl)
    * use "executes" to perform a "command" operation along with any additional Solana "program/account" information needed to send tx on-chain
    * use "staratlas" to interface with blockchain programs (to "play" Sage Labs)