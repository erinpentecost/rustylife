use crate::display;
use crate::inputs;

pub trait Game {
    fn handle_input(&mut self, _time: f64, map: &inputs::InputMap);
    fn simulate(&mut self, _time: f64);
    fn render(&mut self, time: f64, screen: &mut display::Display);
    fn get_target_fps(&self) -> f64;
}
