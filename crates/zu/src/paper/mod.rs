// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

mod variant;

use yew::{classes, function_component, html, AttrValue, Children, Html, Properties};

use crate::styles::CssClass;

// Re-export
pub use variant::Variant;
use zu_util::prop::attr_optional;

pub type Elevation = i32;
pub const ELEVATION_DEFAULT: Elevation = 1;
pub const ELEVATION_MIN: Elevation = 0;
pub const ELEVATION_MAX: Elevation = 24;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    /// Override or extend the styles applied to the component.
    #[prop_or_default]
    pub classes: AttrValue,

    #[prop_or_default]
    pub component: AttrValue,

    #[prop_or_default]
    pub style: AttrValue,

    #[prop_or(ELEVATION_DEFAULT)]
    /// Shadow depth, corresponds to dp in the spec.
    ///
    /// It accepts values between 0 and 24 inclusive.
    pub elevation: Elevation,

    /// If true, rounded corners are disabled.
    #[prop_or(false)]
    pub square: bool,

    #[prop_or_default]
    /// The variant to use.
    pub variant: Variant,
}

#[function_component(Paper)]
pub fn paper(props: &Props) -> Html {
    if props.variant == Variant::Elevation
        && (props.elevation < ELEVATION_MIN || props.elevation > ELEVATION_MAX)
    {
        log::warn!("elevation out of range, expected {ELEVATION_MIN}-{ELEVATION_MAX}");
    };

    let cls = classes!(
        "ZuPaper-root",
        props.classes.as_str().to_owned(),
        props.variant.css_class(),
        if props.square { "" } else { "ZuPaper-rounded" },
        if props.variant == Variant::Elevation {
            format!("ZuPaper-elevation-{}", props.elevation)
        } else {
            String::new()
        }
    );

    let component = if props.component.is_empty() {
        "div"
    } else {
        props.component.as_str()
    };

    html! {
        <@{component.to_owned()} class={cls}
            style={attr_optional(&props.style)}>
            {for props.children.iter()}
        </@>
    }
}
