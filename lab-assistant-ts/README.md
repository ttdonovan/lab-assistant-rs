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
- [ ] a fleet can unload/reload Cargo supplies
- [ ] a fleet can move to sector corrdinates
- [ ] a fleet can survery sector scan for SDU
```

### Example of Usage

See `labs-cmd-center/tests` folder for more detailed examples.

* `solana.test.rs`
* `solana.tx.test.rs`

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
