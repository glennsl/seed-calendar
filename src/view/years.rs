use seed::{prelude::*, *};

use std::rc::Rc;

pub struct YearsView<Ms> {
    from: i32,
    to: i32,
    selected: Option<i32>,
    min: Option<i32>,
    max: Option<i32>,
    on_click: Option<Rc<dyn Fn(i32) -> Ms>>,
}

impl<Ms: 'static> YearsView<Ms> {
    // Constructor

    pub fn new(from: i32, to: i32) -> Self {
        YearsView {
            from,
            to,
            selected: None,
            min: None,
            max: None,
            on_click: None,
        }
    }

    pub fn decade_from(start: i32) -> Self {
        YearsView {
            from: start - 1,
            to: start + 10,
            selected: None,
            min: Some(start),
            max: Some(start + 9),
            on_click: None,
        }
    }

    // Builder functions

    pub fn with_selected(mut self, year: i32) -> Self {
        self.selected = Some(year);
        self
    }

    pub fn on_click(mut self, handler: impl FnOnce(i32) -> Ms + Clone + 'static) -> Self {
        self.on_click = Some(Rc::new(move |year| handler.clone()(year)));
        self
    }

    // Consumers

    pub fn into_node(self) -> Node<Ms> {
        ul![
            C!["seed-calendar-years-view"],
            (self.from..=self.to).map(|year| {
                li![
                    C![
                        IF!(self.selected == Some(year) => "selected"),
                        IF!(self.min.is_some() && self.min > Some(year) => "disabled"),
                        IF!(self.max.is_some() && self.max < Some(year) => "disabled"),
                    ],
                    self.on_click
                        .clone()
                        .map(|on_click| ev(Ev::Click, move |_| on_click(year))),
                    year
                ]
            })
        ]
    }
}

// UpdateEl

impl<Ms: 'static> UpdateEl<Ms> for YearsView<Ms> {
    fn update_el(self, parent: &mut El<Ms>) {
        parent.add_child(self.into_node());
    }
}
