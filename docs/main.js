/******/ (function(modules) { // webpackBootstrap
/******/ 	// The module cache
/******/ 	var installedModules = {};
/******/
/******/ 	// The require function
/******/ 	function __webpack_require__(moduleId) {
/******/
/******/ 		// Check if module is in cache
/******/ 		if(installedModules[moduleId]) {
/******/ 			return installedModules[moduleId].exports;
/******/ 		}
/******/ 		// Create a new module (and put it into the cache)
/******/ 		var module = installedModules[moduleId] = {
/******/ 			i: moduleId,
/******/ 			l: false,
/******/ 			exports: {}
/******/ 		};
/******/
/******/ 		// Execute the module function
/******/ 		modules[moduleId].call(module.exports, module, module.exports, __webpack_require__);
/******/
/******/ 		// Flag the module as loaded
/******/ 		module.l = true;
/******/
/******/ 		// Return the exports of the module
/******/ 		return module.exports;
/******/ 	}
/******/
/******/
/******/ 	// expose the modules object (__webpack_modules__)
/******/ 	__webpack_require__.m = modules;
/******/
/******/ 	// expose the module cache
/******/ 	__webpack_require__.c = installedModules;
/******/
/******/ 	// define getter function for harmony exports
/******/ 	__webpack_require__.d = function(exports, name, getter) {
/******/ 		if(!__webpack_require__.o(exports, name)) {
/******/ 			Object.defineProperty(exports, name, { enumerable: true, get: getter });
/******/ 		}
/******/ 	};
/******/
/******/ 	// define __esModule on exports
/******/ 	__webpack_require__.r = function(exports) {
/******/ 		if(typeof Symbol !== 'undefined' && Symbol.toStringTag) {
/******/ 			Object.defineProperty(exports, Symbol.toStringTag, { value: 'Module' });
/******/ 		}
/******/ 		Object.defineProperty(exports, '__esModule', { value: true });
/******/ 	};
/******/
/******/ 	// create a fake namespace object
/******/ 	// mode & 1: value is a module id, require it
/******/ 	// mode & 2: merge all properties of value into the ns
/******/ 	// mode & 4: return value when already ns object
/******/ 	// mode & 8|1: behave like require
/******/ 	__webpack_require__.t = function(value, mode) {
/******/ 		if(mode & 1) value = __webpack_require__(value);
/******/ 		if(mode & 8) return value;
/******/ 		if((mode & 4) && typeof value === 'object' && value && value.__esModule) return value;
/******/ 		var ns = Object.create(null);
/******/ 		__webpack_require__.r(ns);
/******/ 		Object.defineProperty(ns, 'default', { enumerable: true, value: value });
/******/ 		if(mode & 2 && typeof value != 'string') for(var key in value) __webpack_require__.d(ns, key, function(key) { return value[key]; }.bind(null, key));
/******/ 		return ns;
/******/ 	};
/******/
/******/ 	// getDefaultExport function for compatibility with non-harmony modules
/******/ 	__webpack_require__.n = function(module) {
/******/ 		var getter = module && module.__esModule ?
/******/ 			function getDefault() { return module['default']; } :
/******/ 			function getModuleExports() { return module; };
/******/ 		__webpack_require__.d(getter, 'a', getter);
/******/ 		return getter;
/******/ 	};
/******/
/******/ 	// Object.prototype.hasOwnProperty.call
/******/ 	__webpack_require__.o = function(object, property) { return Object.prototype.hasOwnProperty.call(object, property); };
/******/
/******/ 	// __webpack_public_path__
/******/ 	__webpack_require__.p = "";
/******/
/******/
/******/ 	// Load entry module and return exports
/******/ 	return __webpack_require__(__webpack_require__.s = "./src/main.js");
/******/ })
/************************************************************************/
/******/ ({

/***/ "./src/main.js":
/*!*********************!*\
  !*** ./src/main.js ***!
  \*********************/
/*! no static exports found */
/***/ (function(module, exports) {

eval("//////////////////\n// CANVAS SETUP //\n//////////////////\n\n// Grab the display canvas\nconst renderCanvas = document.getElementById(\"lifegasm-canvas\");\n\n// Prep drawing canvas.\nfunction canvasInit() {\n  console.log(\"Initializing canvas.\")\n\n  // Make sure context menu is turned off.\n  renderCanvas.setAttribute(\"oncontextmenu\", \"return false;\");\n\n  console.log('Getting cursor offsets...')\n  // For calculating cursor offsets.\n  window.canvasCssWidth = getComputedStyle(renderCanvas).getPropertyValue('width').slice(0, -2);\n  window.canvasCssHeight = getComputedStyle(renderCanvas).getPropertyValue('height').slice(0, -2);\n}\n\nnew ResizeObserver(canvasInit).observe(renderCanvas);\n\n// Force focus onto the canvas.\nfunction focusOnCanvas() {\n  renderCanvas.setAttribute('tabindex', '0');\n  renderCanvas.focus();\n}\n\n///////////////////\n// INPUT CAPTURE //\n///////////////////\n\n// The HTML5 canvas has a normal width and height that is internally exposed to\n// it. These are the .width and .height components in the tag.\n// However, canvases can be further scaled by CSS, which stretch the image while\n// leaving the number of 'pixels' the same in the canvas.\n// This messes up our cursor location reporting, so we need to scale back from\n// CSS-mangled size to real size.\nfunction scaleCursor(obj, x, y) {\n  let scaledX = Math.trunc(x * obj.width / window.canvasCssWidth);\n  let scaledY = Math.trunc(y * obj.height / window.canvasCssHeight);\n  return { x: scaledX, y: scaledY };\n}\n\n////////////////\n// WEB WORKER //\n////////////////\n\nasync function main() {\n  // Set up a one-time listener so we can wait until the worker is loaded.\n  const loaded = w =>\n    new Promise(r => w.addEventListener(\"message\", r, { once: true }));\n\n  // Instantiate a new background web worker that will\n  // contain the WASM code.\n  const worker = new Worker('./worker.js');\n\n  // Wait for the worker to load\n  await loaded(worker);\n\n  // Hand over the canvas to worker\n  try {\n    var handover = renderCanvas.transferControlToOffscreen();\n    worker.postMessage({ eventType: \"CANVAS\", eventData: handover }, [handover]);\n  } catch (e) {\n    console.error(e);\n    // https://bugzilla.mozilla.org/show_bug.cgi?id=1390089\n    alert(\"Unsupported browser.\");\n  }\n\n  // Forward input to the worker.\n  var mouseOffset = 230;\n  renderCanvas.addEventListener(\"mousedown\", function (e) {\n    let scaled = scaleCursor(renderCanvas, e.offsetX, e.offsetY);\n    worker.postMessage({ eventType: \"INPUT\", eventData: { MouseEvent: { id: e.which + mouseOffset, pressed: true, x: scaled.x, y: scaled.y } } });\n  });\n  renderCanvas.addEventListener(\"mousemove\", function (e) {\n    let scaled = scaleCursor(renderCanvas, e.offsetX, e.offsetY);\n    worker.postMessage({ eventType: \"INPUT\", eventData: { MouseEvent: { id: e.which + mouseOffset, pressed: true, x: scaled.x, y: scaled.y } } });\n    focusOnCanvas();\n  });\n  renderCanvas.addEventListener(\"mouseup\", function (e) {\n    let scaled = scaleCursor(renderCanvas, e.offsetX, e.offsetY);\n    worker.postMessage({ eventType: \"INPUT\", eventData: { MouseEvent: { id: e.which + mouseOffset, pressed: false, x: scaled.x, y: scaled.y } } });\n  });\n  renderCanvas.addEventListener(\"keydown\", function (e) {\n    if (e.keyCode < mouseOffset) {\n      worker.postMessage({ eventType: \"INPUT\", eventData: { KeyEvent: { id: e.keyCode, pressed: true } } });\n    }\n  }, true);\n\n  renderCanvas.addEventListener(\"keyup\", function (e) {\n    if (e.keyCode < mouseOffset) {\n      worker.postMessage({ eventType: \"INPUT\", eventData: { KeyEvent: { id: e.keyCode, pressed: false } } });\n    }\n  }, true);\n}\n\nmain();\n\n//# sourceURL=webpack:///./src/main.js?");

/***/ })

/******/ });