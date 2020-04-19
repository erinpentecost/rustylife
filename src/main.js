//////////////////
// CANVAS SETUP //
//////////////////

// Grab the display canvas
const renderCanvas = document.getElementById("lifegasm-canvas");

// Prep drawing canvas.
function canvasInit() {
  console.log("Initializing canvas.")

  // Make sure context menu is turned off.
  renderCanvas.setAttribute("oncontextmenu", "return false;");

  console.log('Getting cursor offsets...')
  // For calculating cursor offsets.
  window.canvasCssWidth = getComputedStyle(renderCanvas).getPropertyValue('width').slice(0, -2);
  window.canvasCssHeight = getComputedStyle(renderCanvas).getPropertyValue('height').slice(0, -2);
}

new ResizeObserver(canvasInit).observe(renderCanvas);

// Force focus onto the canvas.
function focusOnCanvas() {
  renderCanvas.setAttribute('tabindex', '0');
  renderCanvas.focus();
}

///////////////////
// INPUT CAPTURE //
///////////////////

// The HTML5 canvas has a normal width and height that is internally exposed to
// it. These are the .width and .height components in the tag.
// However, canvases can be further scaled by CSS, which stretch the image while
// leaving the number of 'pixels' the same in the canvas.
// This messes up our cursor location reporting, so we need to scale back from
// CSS-mangled size to real size.
function scaleCursor(obj, x, y) {
  let scaledX = Math.trunc(x * obj.width / window.canvasCssWidth);
  let scaledY = Math.trunc(y * obj.height / window.canvasCssHeight);
  return { x: scaledX, y: scaledY };
}

////////////////
// WEB WORKER //
////////////////

async function main() {
  // Set up a one-time listener so we can wait until the worker is loaded.
  const loaded = w =>
    new Promise(r => w.addEventListener("message", r, { once: true }));

  // Instantiate a new background web worker that will
  // contain the WASM code.
  const worker = new Worker('./worker.js');

  // Wait for the worker to load
  await loaded(worker);

  // Hand over the canvas to worker
  try {
    var handover = renderCanvas.transferControlToOffscreen();
    worker.postMessage({ eventType: "CANVAS", eventData: handover }, [handover]);
  } catch (e) {
    console.error(e);
    // https://bugzilla.mozilla.org/show_bug.cgi?id=1390089
    alert("Unsupported browser.");
  }

  // Forward input to the worker.
  var mouseOffset = 230;
  renderCanvas.addEventListener("mousedown", function (e) {
    let scaled = scaleCursor(renderCanvas, e.offsetX, e.offsetY);
    worker.postMessage({ eventType: "INPUT", eventData: { MouseEvent: { id: e.which + mouseOffset, pressed: true, x: scaled.x, y: scaled.y } } });
  });
  renderCanvas.addEventListener("mousemove", function (e) {
    let scaled = scaleCursor(renderCanvas, e.offsetX, e.offsetY);
    worker.postMessage({ eventType: "INPUT", eventData: { MouseEvent: { id: e.which + mouseOffset, pressed: true, x: scaled.x, y: scaled.y } } });
    focusOnCanvas();
  });
  renderCanvas.addEventListener("mouseup", function (e) {
    let scaled = scaleCursor(renderCanvas, e.offsetX, e.offsetY);
    worker.postMessage({ eventType: "INPUT", eventData: { MouseEvent: { id: e.which + mouseOffset, pressed: false, x: scaled.x, y: scaled.y } } });
  });
  renderCanvas.addEventListener("keydown", function (e) {
    if (e.keyCode < mouseOffset) {
      worker.postMessage({ eventType: "INPUT", eventData: { KeyEvent: { id: e.keyCode, pressed: true } } });
    }
  }, true);

  renderCanvas.addEventListener("keyup", function (e) {
    if (e.keyCode < mouseOffset) {
      worker.postMessage({ eventType: "INPUT", eventData: { KeyEvent: { id: e.keyCode, pressed: false } } });
    }
  }, true);
}

main();