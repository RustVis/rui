// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(PrecisionManufacturingRounded)]
pub fn precision_manufacturing_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("PrecisionManufacturingRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M19.93,8.35l-3.6,1.68L14,7.7V6.3l2.33-2.33l3.6,1.68c0.38,0.18,0.82,0.01,1-0.36c0.18-0.38,0.01-0.82-0.36-1l-3.92-1.83 c-0.38-0.18-0.83-0.1-1.13,0.2L13.78,4.4C13.6,4.16,13.32,4,13,4c-0.55,0-1,0.45-1,1v1H8.82C8.34,4.66,6.96,3.75,5.4,4.06 C4.23,4.29,3.27,5.25,3.05,6.42C2.8,7.76,3.45,8.96,4.48,9.58L7.08,18H5.5C4.67,18,4,18.67,4,19.5v0C4,20.33,4.67,21,5.5,21h10 c0.83,0,1.5-0.67,1.5-1.5v0c0-0.83-0.67-1.5-1.5-1.5h-2.12L8.41,8.77C8.58,8.53,8.72,8.28,8.82,8H12v1c0,0.55,0.45,1,1,1 c0.32,0,0.6-0.16,0.78-0.4l1.74,1.74c0.3,0.3,0.75,0.38,1.13,0.2l3.92-1.83c0.38-0.18,0.54-0.62,0.36-1 C20.75,8.34,20.31,8.17,19.93,8.35z M6,8C5.45,8,5,7.55,5,7c0-0.55,0.45-1,1-1s1,0.45,1,1C7,7.55,6.55,8,6,8z"/>
        </SvgIcon>
    }
}
