// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

use yew::{classes, function_component, html, AttrValue, Children, Classes, Html, Properties};
use zu_util::prop::ToAttr;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub aria_label: AttrValue,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: Classes,

    #[prop_or(false)]
    pub error: bool,

    /// Display group of elements in a compact row.
    #[prop_or(false)]
    pub row: bool,

    #[prop_or_default]
    pub style: AttrValue,
}

#[function_component(FormGroup)]
pub fn form_group(props: &Props) -> Html {
    let root_cls = classes!(
        "ZuFormGroup-root",
        if props.row { "ZuFormGroup-row" } else { "" },
        if props.error { "ZuFormGroup-error" } else { "" },
        props.classes.clone(),
    );

    html! {
        <div class={root_cls}
            style={props.style.to_attr()}
            aria-label={props.aria_label.to_attr()}>
            {for props.children.iter()}
        </div>
    }
}
