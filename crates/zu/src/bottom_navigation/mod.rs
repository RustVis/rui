// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use yew::{function_component, html, AttrValue, Callback, Children, Event, Html, Properties};
use zu_util::prop;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: AttrValue,

    #[prop_or_default]
    pub component: AttrValue,

    /// Callback fired when the value changes.
    pub on_change: Option<Callback<(Event, i32), ()>>,

    /// If true, all BottomNavigationActions will show their labels.
    #[prop_or(false)]
    pub show_labels: bool,

    #[prop_or_default]
    pub style: AttrValue,
}

#[function_component(BottomNavigation)]
pub fn bottom_navigation(props: &Props) -> Html {
    let root_cls = "ZuBottomNavigation";
    let component = if props.component.is_empty() {
        "div"
    } else {
        props.component.as_str()
    };

    // TODO(Shaohua): Add on_change, value and show_label to children props.

    html! {
        <@{component.to_owned()} class={root_cls}
            style={prop::attr_optional(&props.style)}>
            { for props.children.iter()}
        </@>
    }
}
