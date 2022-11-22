use seed::{prelude::*, *};

use std::rc::Rc;

pub struct MonthsView<Ms> {
    selected: Option<u32>,
    on_click: Option<Rc<dyn Fn(u32) -> Ms>>,
    locale: String,
}

impl<Ms: 'static> MonthsView<Ms> {
    // Constructor

    pub fn new() -> Self {
        MonthsView {
            selected: None,
            on_click: None,
            locale: "en-US".into(),
        }
    }

    // Builder functions

    pub fn with_selected(mut self, month: u32) -> Self {
        self.selected = Some(month);
        self
    }

    pub fn on_click(mut self, handler: impl FnOnce(u32) -> Ms + Clone + 'static) -> Self {
        self.on_click = Some(Rc::new(move |year| handler.clone()(year)));
        self
    }

    // Consumers

    pub fn into_node(self) -> Node<Ms> {
        ul![
            C!["seed-calendar-months-view"],
            (1..=12).map(|month| {
                li![
                    C![IF!(self.selected == Some(month) => "selected"),],
                    self.on_click
                        .clone()
                        .map(|on_click| ev(Ev::Click, move |_| on_click(month))),
                    helpers::format_month(month, &self.locale),
                ]
            })
        ]
    }
}

// UpdateEl

impl<Ms: 'static> UpdateEl<Ms> for MonthsView<Ms> {
    fn update_el(self, parent: &mut El<Ms>) {
        parent.add_child(self.into_node());
    }
}

// Helpers

mod helpers {
    use chrono::NaiveDate;

    pub fn format_month(month: u32, locale: &str) -> String {
        use crate::util::intl;
        use js_sys::*;
        use wasm_bindgen::prelude::*;

        let opts = Object::new();
        Reflect::set(&opts, &JsValue::from("month"), &JsValue::from("short")).unwrap();

        let formatter = intl::DateTimeFormat::new(&Array::of1(&JsValue::from(locale)), &opts);

        let datetime = NaiveDate::from_ymd_opt(1970, month, 1)
            .unwrap()
            .and_hms_opt(12, 0, 0)
            .unwrap();
        let js_date = Date::new(&JsValue::from(datetime.timestamp_millis() as f64));

        formatter.format(&js_date).as_string().unwrap()
    }
}
