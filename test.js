import init, {add} from "./rust-lib-esm/pkg/rust_lib_esm.js";

/*init().then(wasm => {
  window.console.log(wasm)
  window.console.log(wasm.add(423, 4322))
})*/
await init()
window.console.log(add(423, 4322))
