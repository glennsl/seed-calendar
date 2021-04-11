use chrono::NaiveDate;
use seed::{prelude::*, *};

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        start: None,
        end: None,
    }
}
// MODEL

struct Model {
    start: Option<NaiveDate>,
    end: Option<NaiveDate>,
}

// UPDATE

enum Msg {
    SelectDate(NaiveDate),
}

#[allow(clippy::needless_pass_by_value)]
fn update(msg: Msg, model: &mut Model, _orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::SelectDate(date) => match (model.start, model.end) {
            (None, None) => model.start = Some(date),
            (Some(_), None) => model.end = Some(date),
            (None, Some(_)) => model.start = Some(date),
            (Some(_), Some(_)) => {
                model.start = Some(date);
                model.end = None
            }
        },
    }
}

// VIEW

fn view(model: &Model) -> impl IntoNodes<Msg> {
    div![seed_calendar::view::MonthView::new(2021, 4)
        .maybe_with_selection(model.start, model.end)
        .show_week_numbers()
        .on_click(Msg::SelectDate)]
}

// START

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
