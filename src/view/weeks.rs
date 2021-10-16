use chrono::{Datelike, IsoWeek, NaiveDate, Weekday};
use seed::{prelude::*, *};
use std::rc::Rc;

use crate::view::month;

pub struct WeeksView<Ms> {
    year: i32,
    month: u32,
    selection: Selection,
    on_click: Option<Rc<dyn Fn(IsoWeek) -> Ms>>,
    first_weekday: Weekday,
    show_weekdays: bool,
    locale: String,
}

enum Selection {
    None,
    Single(IsoWeek),
    Range(IsoWeek, IsoWeek),
}

impl<Ms: 'static> WeeksView<Ms> {
    // Constructor

    pub fn new(year: i32, month: u32) -> Self {
        WeeksView {
            year,
            month,
            selection: Selection::None,
            on_click: None,
            first_weekday: Weekday::Mon,
            show_weekdays: false,
            locale: String::from("en-US"),
        }
    }

    // Builder functions

    pub fn with_selected(mut self, date: IsoWeek) -> Self {
        self.selection = Selection::Single(date);
        self
    }

    pub fn with_selection(mut self, start: IsoWeek, end: IsoWeek) -> Self {
        self.selection = if start == end {
            Selection::Single(start)
        } else if start > end {
            Selection::Range(end, start)
        } else {
            Selection::Range(start, end)
        };
        self
    }

    pub fn maybe_with_selection(self, start: Option<IsoWeek>, end: Option<IsoWeek>) -> Self {
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

    pub fn with_locale(mut self, locale: impl Into<String>) -> Self {
        self.locale = locale.into();
        self
    }

    pub fn show_weekdays(mut self) -> Self {
        self.show_weekdays = true;
        self
    }

    pub fn on_click(mut self, handler: impl FnOnce(IsoWeek) -> Ms + Clone + 'static) -> Self {
        self.on_click = Some(Rc::new(move |date| handler.clone()(date)));
        self
    }

    // Consumers

    pub fn into_node(self) -> Node<Ms> {
        let month_model = month::MonthView {
            year: self.year,
            month: self.month,
            selection: match self.selection {
                Selection::None => month::Selection::None,
                Selection::Single(week) => month::Selection::Single(NaiveDate::from_isoywd(
                    week.year(),
                    week.week(),
                    self.first_weekday,
                )),
                Selection::Range(start, end) => month::Selection::Range(
                    NaiveDate::from_isoywd(start.year(), start.week(), self.first_weekday),
                    NaiveDate::from_isoywd(end.year(), end.week(), self.first_weekday),
                ),
            },
            on_click: match self.on_click {
                Some(handler) => Some(Rc::new(move |date: NaiveDate| handler(date.iso_week()))),
                None => None,
            },
            first_weekday: self.first_weekday,
            show_week_numbers: true,
            show_weekdays: self.show_weekdays,
            locale: self.locale,
        };

        div![C!["seed-calendar-weeks-view"], month_model]
    }
}

// UpdateEl

impl<Ms: 'static> UpdateEl<Ms> for WeeksView<Ms> {
    fn update_el(self, parent: &mut El<Ms>) {
        parent.add_child(self.into_node());
    }
}
