## Setup

Okay so this has been HEAVILY modified for my personal use. Here's barely an ad-hoc solution to build everything up.

```sh
npm install
make wasm
make build
cd test
npm install
cd test
webpack
```
Then copy `dist/wypst.wasm` to `test/dist/wypst.wasm` for some reason.

Then the test at `test/dist` works.
