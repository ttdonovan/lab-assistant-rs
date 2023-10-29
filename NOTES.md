# Notes

## WASM

    * https://rustwasm.github.io/docs/book/introduction.html
    * https://rustwasm.github.io/docs/wasm-pack/
    * https://solana-labs.github.io/solana-web3.js/

```
node -v
v20.3.0

cd wasm-lab-assistant
wasm-pack build

# npm init wasm-app app

cd app
npm install
npm run start
# open localhost:8080

# after making changes, rerun:
# wasm-pack build
```

## Resources

    * https://www.rapidtables.com/convert/number/hex-to-decimal.html

## SAGE

From: @mosh

> Something else to keep in mind with loading/unloading is that the Starbase can have more than one cargo pod (the extra pods show up if you happen to disband a fleet there and are actually the pods formerly owned by the fleet) which just makes this more complex.  If you find multiple pods at a Starbase best thing to do is remove the extra pods (there is a clean pods function in the SAGE bindings utils)