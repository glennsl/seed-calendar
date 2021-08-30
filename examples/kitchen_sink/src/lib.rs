use chrono::{IsoWeek, NaiveDate};
use seed::{prelude::*, *};

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        year: 2021,
        month: 4,
        start: None,
        end: None,
        start_week: None,
        end_week: None,
    }
}
// MODEL

struct Model {
    year: i32,
    month: u32,
    start: Option<NaiveDate>,
    end: Option<NaiveDate>,
    start_week: Option<IsoWeek>,
    end_week: Option<IsoWeek>,
}

// UPDATE

enum Msg {
    SelectYear(i32),
    SelectMonth(u32),
    SelectDate(NaiveDate),
    SelectWeek(IsoWeek),
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
        Msg::SelectWeek(week) => match (model.start_week, model.end_week) {
            (None, None) => model.start_week = Some(week),
            (Some(_), None) => model.end_week = Some(week),
            (None, Some(_)) => model.start_week = Some(week),
            (Some(_), Some(_)) => {
                model.start_week = Some(week);
                model.end_week = None
            }
        },
    }
}

// VIEW

fn view(model: &Model) -> impl IntoNodes<Msg> {
    div![
        h4!["Month"],
        seed_calendar::view::MonthView::new(model.year, model.month)
            .maybe_with_selection(model.start, model.end)
            .show_week_numbers()
            .show_weekdays()
            .on_click(Msg::SelectDate),
        h4!["Weeks"],
        seed_calendar::view::WeeksView::new(model.year, model.month)
            .maybe_with_selection(model.start_week, model.end_week)
            .show_weekdays()
            .on_click(Msg::SelectWeek),
        h4!["Year"],
        seed_calendar::view::YearsView::decade_from(2010)
            .with_selected(model.year)
            .on_click(Msg::SelectYear),
        h4!["Months"],
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
