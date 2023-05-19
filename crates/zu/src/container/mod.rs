// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use yew::{classes, function_component, html, AttrValue, Children, Html, Properties};

use crate::styles::size::Size;

#[must_use]
pub const fn max_width_cls(size: Option<Size>) -> &'static str {
    match size {
        Some(Size::XSmall) => "ZuContainer-maxWidthXs",
        Some(Size::Small) => "ZuContainer-maxWidthSm",
        Some(Size::Middle) => "ZuContainer-maxWidthMd",
        Some(Size::Large) => "ZuContainer-maxWidthLg",
        Some(Size::XLarge) => "ZuContainer-maxWidthXl",
        Some(Size::Num(_)) | None => "",
    }
}

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

    // TODO(Shaohua): Merge cls_list.
    let mut cls_list = vec!["ZuContainer-root", max_width_cls(props.max_width)];
    if props.disable_gutters {
        cls_list.push("ZuContainer-disableGutters");
    } else {
        cls_list.push("ZuContainer-enableGutters");
    }
    if props.fixed {
        cls_list.push("ZuContainer-fixed");
    }
    let cls = classes!(cls_list, &props.classes);

    // TODO(Shaohua): Merge style
    let style = if let Some(Size::Num(num)) = props.max_width {
        let lst = [props.style.as_str(), &format!("max-width: {num}px")];
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
