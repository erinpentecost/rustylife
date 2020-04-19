
import("../pkg/").then(lifegasm => {
    // The offscreened canvas goes here.
    var canvas = null;

    // Set up the WASM
    var universe = null;

    // Game loop definition.
    const tick = (stamp) => {
        // Set up next frame.
        requestAnimationFrame(tick);

        // Skip until ready.
        if (universe) {
            try {
                universe.tick(stamp);
            } catch (err) {
                console.error(err);
                universe = null;
                throw (err);
            }

        }
    }

    // Sign up for messages from the main (UI) thread.
    self.addEventListener('message', function (e) {
        const { eventType, eventData } = e.data;
        if ((eventType === "INPUT") && (universe)) {
            //console.info("INPUT ");
            universe.add_input(eventData);
        } else if (eventType === "CANVAS") {
            console.info("CANVAS START");
            let init = (canvas == null);
            canvas = eventData;
            if (init) {
                console.log("Starting up wasm...")
                universe = lifegasm.Universe.new(canvas.getContext('2d'));
                requestAnimationFrame(tick);
            } else {
                console.log("Canvas change!")
                universe.reset_render_context(canvas.getContext('2d'));
            }
            console.info("CANVAS END");
        }
    }, false);

    // Let the main thread know that we are done loading.
    postMessage("loaded");
});

