mod display;
mod game;
mod games;
mod inputs;
mod scope;
mod stats;
mod util;
use wasm_bindgen::{prelude::*, JsValue};

extern crate fixedbitset;
extern crate rgb;
extern crate strum;
extern crate web_sys;

#[macro_use]
extern crate strum_macros;

#[macro_use]
extern crate log;

#[macro_use]
extern crate serde_derive;

// This function is automatically invoked after the wasm module is instantiated.
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    Ok(())
}

#[wasm_bindgen]
pub struct Stats {
    pub tick: f64,
}

#[wasm_bindgen]
pub struct Universe {
    game: Box<dyn game::Game>,
    pending_inputs: Vec<inputs::InputEvent>,
    input_map: Box<inputs::InputMap>,
    screen: Box<display::Display>,
    last_timestamp: f64,
    lag: f64,
    tick_stats: stats::Samples,
}

/// Public methods, exported to JavaScript.
#[wasm_bindgen]
impl Universe {
    pub fn new(render_context: web_sys::CanvasRenderingContext2d) -> Universe {
        util::init_panic();
        util::init_log();

        //let game: Box<dyn game::Game> = Box::new(games::demo::Demo::new());
        let game: Box<dyn game::Game> = Box::new(games::life::Life::new());

        info!("Created game object.");

        Universe {
            game,
            pending_inputs: Vec::with_capacity(50 as usize),
            input_map: Box::new(inputs::InputMap::new()),
            screen: Box::new(display::Display::new(render_context)),
            last_timestamp: 0 as f64,
            lag: 0 as f64,
            tick_stats: stats::Samples::new(100),
        }
    }

    pub fn add_input(&mut self, input_events: &JsValue) {
        let des_input = input_events.into_serde::<inputs::InputEvent>();
        match des_input {
            Ok(inps) => self.pending_inputs.push(inps),
            Err(_) => error!("Can't parse input."),
        };
    }

    pub fn tick(&mut self, time: f64) {
        // Desired step lag in ms
        let target_lag: f64 = 1000.0 / self.game.get_target_fps();

        // Track time that has elapsed
        if self.last_timestamp <= 0 as f64 {
            self.last_timestamp = time;
        }
        let delta = time - self.last_timestamp;
        self.tick_stats.add(delta);
        self.last_timestamp = time;
        self.lag += delta;

        // Just quit now if our clock is too fast.
        if self.lag < target_lag {
            return;
        }

        // Build input map.
        self.input_map
            .process_new_events(self.pending_inputs.as_slice());
        self.pending_inputs.clear();

        // Advance game state with fixed step.
        let mut max_frames_done = 5;
        while self.lag >= target_lag {
            // Worst case, we are really super slow.
            if max_frames_done < 0 {
                break;
            }
            // Parse inputs.
            self.game
                .handle_input(time - self.lag, self.input_map.as_ref());
            // Newly-hit keys should only be sent once per game loop tick
            self.input_map.clear_hits();
            // Fixed step
            self.game.simulate(time);
            self.lag -= target_lag;
            max_frames_done -= 1;
        }

        // Render once.
        self.render(time);
    }

    pub fn render(&mut self, time: f64) {
        // Draw game image.
        self.game.render(time, self.screen.as_mut());

        // Render to screen!
        self.screen.render();
    }

    pub fn reset_render_context(&mut self, render_context: web_sys::CanvasRenderingContext2d) {
        self.screen = Box::new(display::Display::new(render_context));
    }

    pub fn get_fps(&mut self) -> f64 {
        self.tick_stats.average()
    }
}
