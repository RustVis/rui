// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(EventRepeatSharp)]
pub fn event_repeat_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("EventRepeatSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M21,12V4h-3V2h-2v2H8V2H6v2H3v18h9v-2H5V10h14v2H21z M15.64,20c0.43,1.45,1.77,2.5,3.36,2.5c1.93,0,3.5-1.57,3.5-3.5 s-1.57-3.5-3.5-3.5c-0.95,0-1.82,0.38-2.45,1l1.45,0V18h-4v-4h1.5l0,1.43C16.4,14.55,17.64,14,19,14c2.76,0,5,2.24,5,5s-2.24,5-5,5 c-2.42,0-4.44-1.72-4.9-4L15.64,20z"/>
        </SvgIcon>
    }
}
