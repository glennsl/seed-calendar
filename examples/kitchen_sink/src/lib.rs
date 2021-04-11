use chrono::NaiveDate;
use seed::{prelude::*, *};

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        year: 2021,
        month: 4,
        start: None,
        end: None,
    }
}
// MODEL

struct Model {
    year: i32,
    month: u32,
    start: Option<NaiveDate>,
    end: Option<NaiveDate>,
}

// UPDATE

enum Msg {
    SelectYear(i32),
    SelectMonth(u32),
    SelectDate(NaiveDate),
}

#[allow(clippy::needless_pass_by_value)]
fn update(msg: Msg, model: &mut Model, _orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::SelectYear(year) => model.year = year,
        Msg::SelectMonth(month) => model.month = month,
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
    div![
        seed_calendar::view::MonthView::new(model.year, model.month)
            .maybe_with_selection(model.start, model.end)
            .show_week_numbers()
            .show_weekdays()
            .on_click(Msg::SelectDate),
        seed_calendar::view::YearsView::decade_from(2010)
            .with_selected(model.year)
            .on_click(Msg::SelectYear),
        seed_calendar::view::MonthsView::new()
            .with_selected(model.month)
            .on_click(Msg::SelectMonth),
    ]
}

// START

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
