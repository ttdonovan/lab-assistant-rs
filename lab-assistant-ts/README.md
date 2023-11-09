# Lab Assistant (*WIP*)

**_Warning!_** This project is a work in progress and apis are subject to change. 

Explore a app setup with Bun (and Rust?).

## labs-cmd-center

A TypeScript app that uses the `@staratlas/sage` package.

See [README.md](labs-cmd-center/README.md) for more details.

### Features

```
- [x] a fleet can start/stop mining
- [x] a fleet can dock/undock from Starbase
- [x] a fleet can withdraw/unload cargo (deposit to Starbase)
- [x] a fleet can deposit/reload cargo supplies (withdraw from Starbase)
- [/] a fleet can move to sector coordinates (warp)
    - [ ] movement handler (?)
    - [x] warp cordinate
    - [ ] warp lane (?)
- [ ] a fleet can move to sector coordinates (subwarp)
- [ ] a fleet can survey sector scan for SDU
```

### Example of Usage

#### Tests

See `labs-cmd-center/tests` folder for more detailed examples.

* `solana.test.rs`
* `solana.tx.test.rs`

See `labs-cmd-center/examples` folder for simple instructions.

* Mine Asteroids - `mining.ts`
* Move to Coordinates - `movement.ts`

#### Server

First configure the `labs-cmd-center/.env` file.

```
cd labs-cmd-center
cp .env.sample .env
bun run src/server.ts
```

See [thunder-collection_labs-cmd-center-api.json](docs/thunder-collection_labs-cmd-center-api.json) for more details.

### Developer Notes

#### Bun

* https://bun.sh/
* https://youtu.be/U4JVw8K19uY

#### Windows

```
wsl --help
wsl --list
wsl -d Ubuntu -u <UserName> --system
```

#### Ubuntu

```
su - <username>
```

#### Development

```
cd labs-cmd-center
bun test
```

## Resources

Additional resources.

### Solana/Anchor

* https://docs.solana.com/
* https://solanacookbook.com
