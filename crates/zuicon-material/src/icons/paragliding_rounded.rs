// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ParaglidingRounded)]
pub fn paragliding_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ParaglidingRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M12,17c-1.1,0-2-0.9-2-2s0.9-2,2-2s2,0.9,2,2S13.1,17,12,17z M17.95,14c-0.52,0-0.94,0.4-0.99,0.92 c-0.2,2.03-1.05,2.68-1.48,3.02C14.68,18.54,14,19,12,19s-2.68-0.46-3.48-1.06c-0.43-0.34-1.28-0.99-1.48-3.02 C6.99,14.4,6.57,14,6.05,14c-0.59,0-1.06,0.51-1,1.09c0.22,2.08,1.07,3.47,2.24,4.41c0.5,0.4,1.1,0.7,1.7,0.9L9,24h6v-3.6 c0.6-0.2,1.2-0.5,1.7-0.9c1.17-0.94,2.03-2.32,2.24-4.41C19.01,14.51,18.53,14,17.95,14z M12,0C5.92,0,1,1.9,1,4.25v3.49 C1,8.55,1.88,9,2.56,8.57C2.7,8.48,2.84,8.39,3,8.31L5,13h2l1.5-6.28C9.6,6.58,10.78,6.5,12,6.5s2.4,0.08,3.5,0.22L17,13h2l2-4.69 c0.16,0.09,0.3,0.17,0.44,0.26C22.12,9,23,8.55,23,7.74V4.25C23,1.9,18.08,0,12,0z M5.88,11.24L4.37,7.69 c0.75-0.28,1.6-0.52,2.53-0.71L5.88,11.24z M18.12,11.24L17.1,6.98c0.93,0.19,1.78,0.43,2.53,0.71L18.12,11.24z"/>
        </SvgIcon>
    }
}
