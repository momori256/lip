import * as wasm from "./lp_bg.wasm";
import { __wbg_set_wasm } from "./lp_bg.js";
__wbg_set_wasm(wasm);
export * from "./lp_bg.js";
