// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(Crib)]
pub fn crib(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("Crib"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M18,9h-6V4H8C5.79,4,4,5.79,4,8v6c0,1.1,0.9,2,2,2h2v2.93c-0.61-0.35-1.16-0.78-1.65-1.27l-1.42,1.42 C6.74,20.88,9.24,22,12,22c2.76,0,5.26-1.12,7.07-2.93l-1.42-1.42c-0.49,0.49-1.05,0.92-1.65,1.27V16h2c1.1,0,2-0.9,2-2v-3 C20,9.9,19.1,9,18,9z M14,19.75C13.36,19.91,12.69,20,12,20c-0.69,0-1.36-0.09-2-0.25V16h4V19.75z"/>
        </SvgIcon>
    }
}
