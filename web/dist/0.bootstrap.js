(window["webpackJsonp"] = window["webpackJsonp"] || []).push([[0],{

/***/ "./node_modules/rust-vdom/rust_vdom.js":
/*!*********************************************!*\
  !*** ./node_modules/rust-vdom/rust_vdom.js ***!
  \*********************************************/
/*! exports provided: __wbg_alert_03d883d6e976dd5e, greet */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_alert_03d883d6e976dd5e\", function() { return __wbg_alert_03d883d6e976dd5e; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"greet\", function() { return greet; });\n/* harmony import */ var _rust_vdom_bg__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./rust_vdom_bg */ \"./node_modules/rust-vdom/rust_vdom_bg.wasm\");\n\n\nlet cachedTextDecoder = new TextDecoder('utf-8');\n\nlet cachegetUint8Memory = null;\nfunction getUint8Memory() {\n    if (cachegetUint8Memory === null || cachegetUint8Memory.buffer !== _rust_vdom_bg__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer) {\n        cachegetUint8Memory = new Uint8Array(_rust_vdom_bg__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n    }\n    return cachegetUint8Memory;\n}\n\nfunction getStringFromWasm(ptr, len) {\n    return cachedTextDecoder.decode(getUint8Memory().subarray(ptr, ptr + len));\n}\n\nfunction __wbg_alert_03d883d6e976dd5e(arg0, arg1) {\n    let varg0 = getStringFromWasm(arg0, arg1);\n    alert(varg0);\n}\n/**\n* @returns {void}\n*/\nfunction greet() {\n    return _rust_vdom_bg__WEBPACK_IMPORTED_MODULE_0__[\"greet\"]();\n}\n\n\n\n//# sourceURL=webpack:///./node_modules/rust-vdom/rust_vdom.js?");

/***/ }),

/***/ "./node_modules/rust-vdom/rust_vdom_bg.wasm":
/*!**************************************************!*\
  !*** ./node_modules/rust-vdom/rust_vdom_bg.wasm ***!
  \**************************************************/
/*! exports provided: memory, greet */
/***/ (function(module, exports, __webpack_require__) {

eval("\"use strict\";\n// Instantiate WebAssembly module\nvar wasmExports = __webpack_require__.w[module.i];\n__webpack_require__.r(exports);\n// export exports from WebAssembly module\nfor(var name in wasmExports) if(name != \"__webpack_init__\") exports[name] = wasmExports[name];\n// exec imports from WebAssembly module (for esm order)\n/* harmony import */ var m0 = __webpack_require__(/*! ./rust_vdom */ \"./node_modules/rust-vdom/rust_vdom.js\");\n\n\n// exec wasm module\nwasmExports[\"__webpack_init__\"]()\n\n//# sourceURL=webpack:///./node_modules/rust-vdom/rust_vdom_bg.wasm?");

/***/ }),

/***/ "./src/entry.js":
/*!**********************!*\
  !*** ./src/entry.js ***!
  \**********************/
/*! no exports provided */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var rust_vdom__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! rust-vdom */ \"./node_modules/rust-vdom/rust_vdom.js\");\n\n\nrust_vdom__WEBPACK_IMPORTED_MODULE_0__[\"greet\"]();\n\n//# sourceURL=webpack:///./src/entry.js?");

/***/ })

}]);