// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(MicrowaveSharp)]
pub fn microwave_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("MicrowaveSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M6.8,10.61L5.37,9.19C5.73,8.79,6.59,8,7.75,8c0.8,0,1.39,0.39,1.81,0.67C9.87,8.88,10.07,9,10.25,9 c0.37,0,0.8-0.41,0.95-0.61l1.42,1.42c-0.36,0.4-1.22,1.19-2.37,1.19c-0.79,0-1.37-0.38-1.79-0.66C8.13,10.12,7.94,10,7.75,10 C7.38,10,6.95,10.41,6.8,10.61z M7.75,15c0.19,0,0.38,0.12,0.71,0.34c0.42,0.28,1,0.66,1.79,0.66c1.16,0,2.01-0.79,2.37-1.19 l-1.42-1.42c-0.15,0.2-0.59,0.61-0.95,0.61c-0.18,0-0.38-0.12-0.69-0.33C9.14,13.39,8.55,13,7.75,13c-1.16,0-2.02,0.79-2.38,1.19 l1.42,1.42C6.95,15.41,7.38,15,7.75,15z M22,4v16H2V4H22z M14,6H4v12h10V6z M19,16c0-0.55-0.45-1-1-1c-0.55,0-1,0.45-1,1 c0,0.55,0.45,1,1,1C18.55,17,19,16.55,19,16z M19,12c0-0.55-0.45-1-1-1c-0.55,0-1,0.45-1,1c0,0.55,0.45,1,1,1 C18.55,13,19,12.55,19,12z M19,7h-2v2h2V7z"/>
        </SvgIcon>
    }
}
