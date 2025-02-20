// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(HdrOnSelectRounded)]
pub fn hdr_on_select_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("HdrOnSelectRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M18,18.5v-1c0-0.83-0.67-1.5-1.5-1.5H14c-0.55,0-1,0.45-1,1v4.31c0,0.38,0.31,0.69,0.69,0.69h0.11 c0.38,0,0.69-0.31,0.69-0.69V20h1.1l0.72,1.59c0.11,0.25,0.36,0.41,0.63,0.41h0c0.5,0,0.83-0.51,0.64-0.97L17.1,19.9 C17.6,19.6,18,19.1,18,18.5z M16.5,18.5h-2v-1h2V18.5z M3.5,18h-2v-1.25C1.5,16.34,1.16,16,0.75,16h0C0.34,16,0,16.34,0,16.75v4.5 C0,21.66,0.34,22,0.75,22h0c0.41,0,0.75-0.34,0.75-0.75V19.5h2v1.75C3.5,21.66,3.84,22,4.25,22h0C4.66,22,5,21.66,5,21.25v-4.5 C5,16.34,4.66,16,4.25,16h0c-0.41,0-0.75,0.34-0.75,0.75V18z M10,16H7.5c-0.55,0-1,0.45-1,1v4c0,0.55,0.45,1,1,1H10 c0.82,0,1.5-0.67,1.5-1.5v-3C11.5,16.67,10.82,16,10,16z M10,20.5H8v-3h2V20.5z M23.25,20H22v1.25c0,0.41-0.34,0.75-0.75,0.75l0,0 c-0.41,0-0.75-0.34-0.75-0.75V20h-1.25c-0.41,0-0.75-0.34-0.75-0.75l0,0c0-0.41,0.34-0.75,0.75-0.75h1.25v-1.25 c0-0.41,0.34-0.75,0.75-0.75l0,0c0.41,0,0.75,0.34,0.75,0.75v1.25h1.25c0.41,0,0.75,0.34,0.75,0.75l0,0 C24,19.66,23.66,20,23.25,20z M12,4c2.21,0,4,1.79,4,4s-1.79,4-4,4s-4-1.79-4-4S9.79,4,12,4 M12,2C8.69,2,6,4.69,6,8s2.69,6,6,6 s6-2.69,6-6S15.31,2,12,2z"/>
        </SvgIcon>
    }
}
