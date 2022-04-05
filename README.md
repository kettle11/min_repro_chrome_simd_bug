# min_repro_chrome_simd_bug
A minimal repro of a Chrome / SIMD / Wasm bug

This repository reproduces an bug I stumbled across while debugging a physics issues for a game.

Chrome's WebAssembly SIMD code appears to work inconsistently when SIMD and Wasm threads are enabled. 

## Steps to reproduce:

* Clone this repository
* Host the `web_build` directory locally with these CORS headers set on the local server: 
  `--header Cross-Origin-Opener-Policy='same-origin' --header Cross-Origin-Embedder-Policy='require-corp'`
  
* Go to your locally hosted page **with the developer console closed** and after the page has loaded open the console.
  *  If the bug reproduces it will show `INITIALIZED: 0`.
  *   If it does not reproduce it will show `INITIALIZED: 199.9076385498047`

* If the developer console is opened when the page is loaded this will not reproduce. 

## Code that's going wrong.

This code within `parry` is where things are going wrong:
https://github.com/dimforge/parry/blob/167e0948d5f47107390e6d9c84f67095b6b42c3e/src/partitioning/qbvh.rs#L274

See this issue I opened for more information: https://github.com/dimforge/parry/issues/74
## How to build

Run ./web_build.sh

## Platforms tested

M1 Macbook Air

## Further info

I wrote more permutations of things I tried in this issue: https://github.com/dimforge/parry/issues/74
