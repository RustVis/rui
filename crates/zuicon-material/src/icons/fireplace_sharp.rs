// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(FireplaceSharp)]
pub fn fireplace_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("FireplaceSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M2,2v20h20V2H2z M13.2,13.74c-0.08-0.46-0.07-0.85,0.08-1.28c0.54,1.21,2.15,1.64,1.98,3.18 c-0.19,1.69-2.11,2.37-3.39,1.32c0.76-0.24,1.4-1.04,1.53-1.63C13.52,14.78,13.29,14.29,13.2,13.74z M20,20h-2v-2h-2.02 c0.63-0.84,1.02-1.87,1.02-3c0-1.89-1.09-2.85-1.85-3.37C12.2,9.61,13,7,13,7c-6.73,3.57-6.02,7.47-6,8c0.03,0.96,0.49,2.07,1.23,3 H6v2H4V4h16V20z"/>
        </SvgIcon>
    }
}
