// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

use yew::{classes, function_component, html, AttrValue, Children, Classes, Html, Properties};

use crate::typography::Typography;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: Classes,

    #[prop_or_default]
    pub style: AttrValue,
}

#[function_component(AlertTitle)]
pub fn alert_title(props: &Props) -> Html {
    let root_cls = classes! {
        "ZuAlertTitle-root",
        props.classes.clone(),
    };

    // TODO(Shaohua): Merge properties with props!() macro.

    html! {
        <Typography
            classes={root_cls}
            style={&props.style}
            component="div">
            {for props.children.iter()}
        </Typography>
    }
}
