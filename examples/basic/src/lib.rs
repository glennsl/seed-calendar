use seed::{prelude::*, *};

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {}
}
// MODEL

struct Model {}

// UPDATE

enum Msg {}

#[allow(clippy::needless_pass_by_value)]
fn update(_msg: Msg, _model: &mut Model, _orders: &mut impl Orders<Msg>) {}

// VIEW

fn view(_model: &Model) -> impl IntoNodes<Msg> {
    div![seed_calendar::calendar()]
}

// START

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
