// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(TtySharp)]
pub fn tty_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("TtySharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M20,15v6c-3.28,0-6.35-0.89-9-2.43C8.27,16.99,6.01,14.73,4.43,12C2.89,9.35,2,6.28,2,3h6l1,5l-2.9,2.9 c1.43,2.5,3.5,4.57,6,6L15,14L20,15z M14,6h2V4h-2V6z M13,9h2V7h-2V9z M11,6h2V4h-2V6z M18,7h-2v2h2V7z M19,4h-2v2h2V4z M21,7h-2v2 h2V7z M22,4h-2v2h2V4z M14,12h2v-2h-2V12z M11,12h2v-2h-2V12z M19,10h-2v2h2V10z M22,10h-2v2h2V10z"/>
        </SvgIcon>
    }
}
