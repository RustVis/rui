// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(AppsOutageSharp)]
pub fn apps_outage_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("AppsOutageSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M4,8h4V4H4V8z M10,20h4v-4h-4V20z M4,20h4v-4H4V20z M4,14h4v-4H4V14z M10,14h4v-4h-4V14z M16,20h4v-4h-4V20z M19,0 c-2.76,0-5,2.24-5,5s2.24,5,5,5s5-2.24,5-5S21.76,0,19,0z M19.5,8h-1V7h1V8z M19.5,6h-1V2h1V6z M16,14h4v-2.07 C19.67,11.98,19.34,12,19,12c-1.07,0-2.09-0.24-3-0.68V14z M10,4v4h2.68C12.24,7.09,12,6.07,12,5c0-0.34,0.02-0.67,0.07-1H10z"/>
        </SvgIcon>
    }
}
