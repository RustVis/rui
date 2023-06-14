// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

mod color;
mod content;
mod origin;
mod overlap;
mod variant;

use yew::{classes, function_component, html, AttrValue, Children, Html, Properties};
use zu_util::prop::ToAttr;

use crate::styles::origin::Origin;
use crate::styles::{color::Color, CssClass};
pub use content::Content;
pub use overlap::Overlap;
pub use variant::Variant;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or(Origin::top_right())]
    pub anchor_origin: Origin,

    /// The content rendered within the badge.
    #[prop_or_default]
    pub content: Option<Content>,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub color: Color,

    #[prop_or_default]
    pub component: AttrValue,

    /// If true, the badge is invisible.
    #[prop_or(false)]
    pub invisible: bool,

    /// Max count to show.
    #[prop_or(99)]
    pub max: i32,

    /// Wrapped shape the badge should overlap.
    #[prop_or_default]
    pub overlap: Overlap,

    /// Controls whether the badge is hidden when badgeContent is zero.
    #[prop_or(false)]
    pub show_zero: bool,

    #[prop_or_default]
    pub style: AttrValue,

    // TODO(Shaohua): Support custom inline style
    //pub style: String,
    /// The variant to use.
    #[prop_or_default]
    pub variant: Variant,
}

#[function_component(Badge)]
pub fn badge(props: &Props) -> Html {
    let badge_cls = classes!(
        "ZuBadge-badge",
        props.variant.css_class(),
        if props.invisible {
            "ZuBadge-invisible"
        } else {
            ""
        },
        origin::css_class(props.anchor_origin),
        props.overlap.css_class(),
        color::color_class(props.color),
    );

    let component = if props.component.is_empty() {
        "span"
    } else {
        props.component.as_str()
    };

    // TODO(Shaohua): Use invisible property.
    let _invisible = props.invisible || props.content.is_none() || props.variant != Variant::Dot;

    let display_value = if props.variant == Variant::Standard {
        match &props.content {
            None => None,
            Some(Content::Str(s)) => Some(s.clone()),
            Some(Content::Num(num)) => {
                if *num > props.max {
                    Some(format!("{}+", props.max))
                } else {
                    Some(num.to_string())
                }
            }
        }
    } else {
        None
    };

    html! {
        <@{component.to_owned()} class="ZuBadge-root" style={props.style.to_attr()}>
            {for props.children.iter()}
            <span class={badge_cls}>{display_value}</span>
        </@>
    }
}
