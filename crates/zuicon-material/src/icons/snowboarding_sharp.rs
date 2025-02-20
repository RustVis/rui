// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SnowboardingSharp)]
pub fn snowboarding_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SnowboardingSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M14,3c0-1.1,0.9-2,2-2s2,0.9,2,2c0,1.1-0.9,2-2,2S14,4.1,14,3z M21.4,20.09c-0.23-0.05-0.46,0.02-0.64,0.17 c-0.69,0.6-1.64,0.88-2.6,0.67L17,20.69l-1-6.19l-3.32-2.67l1.8-2.89C15.63,10.78,17.68,12,20,12v-2c-1.85,0-3.44-1.12-4.13-2.72 l-0.52-1.21C15.16,5.64,14.61,5,13.7,5H8L5.5,9l1.7,1.06L9.1,7h2.35L8.5,11.7l1,4.3L6,18.35l-0.47-0.1c-0.96-0.2-1.71-0.85-2.1-1.67 c-0.1-0.21-0.28-0.37-0.51-0.42c-0.43-0.09-0.82,0.2-0.9,0.58C1.98,16.88,2,17.05,2.07,17.2c0.58,1.24,1.71,2.2,3.15,2.51 l12.63,2.69c1.44,0.31,2.86-0.11,3.9-1.01c0.13-0.11,0.21-0.26,0.24-0.41C22.06,20.6,21.83,20.18,21.4,20.09z M8.73,18.93l3.02-2.03 l-0.44-3.32l2.84,2.02l0.75,4.64L8.73,18.93z"/>
        </SvgIcon>
    }
}
