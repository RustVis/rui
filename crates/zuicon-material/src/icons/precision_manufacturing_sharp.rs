// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(PrecisionManufacturingSharp)]
pub fn precision_manufacturing_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("PrecisionManufacturingSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M14,10V9.82l2.01,2.01l5.23-2.44l-0.63-1.36l-4.28,2L14,7.7V6.3l2.33-2.33l4.28,2l0.63-1.36l-5.23-2.44L14,4.18V4h-2v2 H8.82C8.4,4.84,7.3,4,6,4C4.34,4,3,5.34,3,7c0,1.1,0.6,2.05,1.48,2.58L7.08,18H4v3h13v-3h-3.62L8.41,8.76 C8.58,8.53,8.72,8.28,8.82,8H12v2H14z M6,8C5.45,8,5,7.55,5,7c0-0.55,0.45-1,1-1s1,0.45,1,1C7,7.55,6.55,8,6,8z"/>
        </SvgIcon>
    }
}
