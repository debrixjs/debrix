<div align="center">

[![Debrix Banner: Efficient progressive component-based Javascript library](https://raw.githubusercontent.com/debrixjs/assets/main/images/banner.svg)](https://debrix.dev)

</div>

# Debrix Compiler

## 🚧 _UNDER DEVELOPMENT_ 🚧

**Only use debrix in it's current state to experiment. Most features are not implemented or tested yet and will break.** Debrix is far from being complete and is under active development. Don't post the library in a forum or similar, I don't want the project to have much attention yet. Ideas and questions are very welcome under the [discussions](https://github.com/debrixjs/debrix/discussions) on github.

## Installation

SemVer allows breaking changes in any release! Make sure to --save-exact to avoid breakning your application.

```
npm install --save-exact @debrix/compiler
```

## Usage

Packages for some third-party tooling exists under the [@debrix](https://www.npmjs.com/search?q=%40debrix) organisation on npm.

The compiler is written in rust. The crate is located under [packages/debrix-compiler/crates](https://github.com/debrixjs/debrix/tree/main/packages/debrix-compiler/crates) on github.

The package exports two distributions of the compiler: wasm and node. The wasm distribution will run in most runtimes, but not in [NodeJS](https://nodejs.org). For NodeJS, use the node distrubiton. The node distrubution is compiled into a native module, which will only run by supported runtimes (NodeJS/[Bun](https://bun.sh/)). Both distributions export the same interfaces.

```js
// For NodeJS only (node native)
import ... from '@debrix/compiler/node'

// For other runtimes (browser, deno, etc.)
import ... from '@debrix/compiler/wasm'
```

The `build` function takes the input string as the first parameter, and options as the optional second parameter. Valid targets are client (default), server, and hydration.

```js
import { build, Target } from '@debrix/compiler/...';
const { source, sourcemap } = await build('...', Target.Client);
```
