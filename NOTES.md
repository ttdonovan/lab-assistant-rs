# Notes

## WASM

    * https://rustwasm.github.io/docs/book/introduction.html
    * https://rustwasm.github.io/docs/wasm-pack/

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