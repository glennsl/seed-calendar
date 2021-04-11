use chrono::{Datelike, Duration, NaiveDate, Weekday};
use seed::{prelude::*, *};
use std::rc::Rc;

pub struct MonthView<Ms> {
    year: i32,
    month: u32,
    selection: Selection,
    on_click: Option<Rc<dyn Fn(NaiveDate) -> Ms>>,
    first_weekday: Weekday,
    show_week_numbers: bool,
}

enum Selection {
    None,
    Single(NaiveDate),
    Range(NaiveDate, NaiveDate),
}

impl<Ms: 'static> MonthView<Ms> {
    // Constructor

    pub fn new(year: i32, month: u32) -> Self {
        MonthView {
            year,
            month,
            selection: Selection::None,
            on_click: None,
            first_weekday: Weekday::Mon,
            show_week_numbers: false,
        }
    }

    // Builder functions

    pub fn with_selected(mut self, date: NaiveDate) -> Self {
        self.selection = Selection::Single(date);
        self
    }

    pub fn with_selection(mut self, start: NaiveDate, end: NaiveDate) -> Self {
        self.selection = if start == end {
            Selection::Single(start)
        } else if start > end {
            Selection::Range(end, start)
        } else {
            Selection::Range(start, end)
        };
        self
    }

    pub fn maybe_with_selection(self, start: Option<NaiveDate>, end: Option<NaiveDate>) -> Self {
        match (start, end) {
            (None, None) => self,
            (Some(date), None) | (None, Some(date)) => self.with_selected(date),
            (Some(start), Some(end)) => self.with_selection(start, end),
        }
    }

    pub fn with_first_weekday(mut self, weekday: Weekday) -> Self {
        self.first_weekday = weekday;
        self
    }

    pub fn show_week_numbers(mut self) -> Self {
        self.show_week_numbers = true;
        self
    }

    pub fn on_click(mut self, handler: impl FnOnce(NaiveDate) -> Ms + Clone + 'static) -> Self {
        self.on_click = Some(Rc::new(move |date| handler.clone()(date)));
        self
    }

    // Consumers

    pub fn into_node(self) -> Node<Ms> {
        let first_of_month = NaiveDate::from_ymd(self.year, self.month, 1);
        let start_date = {
            let start_date = helpers::start_of_week(first_of_month, self.first_weekday);

            if first_of_month < start_date {
                start_date - Duration::weeks(1)
            } else {
                start_date
            }
        };

        let weeks = start_date.iter_weeks().take(6);
        // .take_while(|date| date.month() <= self.month);

        table![
            C!["seed-calendar-month-view"],
            attrs! {
                At::from("role") => "presentation",
            },
            weeks.map(|week| {
                let days = week.iter_days().take(7);

                tr![
                    self.show_week_numbers.then(|| td![
                        C!["week-number"],
                        helpers::week_number(week, self.first_weekday)
                    ]),
                    days.map(|date| {
                        if date.month() == self.month {
                            let on_click = self.on_click.clone();

                            td![button![
                                C![match self.selection {
                                    Selection::Single(selected) if selected == date => "selected",
                                    Selection::Range(start, _) if start == date =>
                                        "selection-start",
                                    Selection::Range(_, end) if end == date => "selection-end",
                                    Selection::Range(start, end) if date > start && date < end =>
                                        "in-selection",
                                    _ => "",
                                }],
                                on_click.map(|on_click| ev(Ev::Click, move |_| on_click(date))),
                                date.day()
                            ]]
                        } else {
                            td![date.day()]
                        }
                    })
                ]
            })
        ]
    }
}

// UpdateEl

impl<Ms: 'static> UpdateEl<Ms> for MonthView<Ms> {
    fn update_el(self, parent: &mut El<Ms>) {
        parent.add_child(self.into_node());
    }
}

// Helpers

mod helpers {
    use chrono::{Datelike, Duration, NaiveDate, Weekday};

    pub fn days_since_earliest_weekday(weekday: Weekday) -> u32 {
        match weekday {
            Weekday::Sat => 0,
            Weekday::Sun => 1,
            Weekday::Mon => 2,
            Weekday::Tue => 3,
            Weekday::Wed => 4,
            Weekday::Thu => 5,
            Weekday::Fri => 6,
        }
    }

    pub fn days_since_start_of_week(date: NaiveDate, first_weekday: Weekday) -> i32 {
        days_since_earliest_weekday(date.weekday()) as i32
            - days_since_earliest_weekday(first_weekday) as i32
    }

    pub fn start_of_week(date: NaiveDate, first_weekday: Weekday) -> NaiveDate {
        let offset = days_since_start_of_week(date, first_weekday);

        date - Duration::days(offset as i64)
    }

    pub fn week_number(date: NaiveDate, first_weekday: Weekday) -> u32 {
        let offset = first_weekday.num_days_from_monday();
        let adjusted_date = date + Duration::days(offset as i64);

        adjusted_date.iso_week().week()
    }
}
