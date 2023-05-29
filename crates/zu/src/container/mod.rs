// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

mod size;

use yew::{classes, function_component, html, AttrValue, Children, Html, Properties};

// Re-export
pub use size::Size;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: String,

    #[prop_or_default]
    pub component: AttrValue,

    /// If true, the left and right padding is removed.
    #[prop_or(false)]
    pub disable_gutters: bool,

    /// Set the max-width to match the min-width of the current breakpoint.
    #[prop_or(false)]
    pub fixed: bool,

    /// Determine the max-width of the container.
    ///
    /// The container width grows with the size of the screen.
    /// Set to None to disable max-width.
    #[prop_or_default]
    pub max_width: Option<Size>,

    #[prop_or_default]
    pub style: AttrValue,
}

#[function_component(Container)]
pub fn container(props: &Props) -> Html {
    let component = if props.component.is_empty() {
        "div"
    } else {
        props.component.as_str()
    };

    let cls = classes!(
        "ZuContainer-root",
        size::max_width_cls(&props.max_width),
        if props.disable_gutters {
            "ZuContainer-disableGutters"
        } else {
            "ZuContainer-enableGutters"
        },
        if props.fixed { "ZuContainer-fixed" } else { "" },
        &props.classes
    );

    let style = if let Some(Size::Str(s)) = &props.max_width {
        let lst = [props.style.as_str(), &format!("max-width: {s}")];
        lst.join(";")
    } else {
        props.style.as_str().to_owned()
    };

    html! {
        <@{component.to_owned()} class={cls} style={style}>
            {for props.children.iter()}
        </@>
    }
}
