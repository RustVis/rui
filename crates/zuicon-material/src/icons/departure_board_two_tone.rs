// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(DepartureBoardTwoTone)]
pub fn departure_board_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("DepartureBoardTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M9.29 6H9c-3.69 0-5.11.46-5.66.99h5.74c.05-.33.12-.67.21-.99zM3 14v4c0 .37.21.62.34.73l.29.27h10.74l.29-.27c.13-.11.34-.36.34-.73v-3.08c-.94-.13-1.81-.45-2.59-.92H3zm2.5 4c-.83 0-1.5-.67-1.5-1.5S4.67 15 5.5 15s1.5.67 1.5 1.5S6.33 18 5.5 18zm8.5-1.5c0 .83-.67 1.5-1.5 1.5s-1.5-.67-1.5-1.5.67-1.5 1.5-1.5 1.5.67 1.5 1.5z" opacity=".3"/><path d="M16 1c-2.39 0-4.49 1.2-5.75 3.02C9.84 4.01 9.43 4 9 4c-4.42 0-8 .5-8 4v10c0 .88.39 1.67 1 2.22V22c0 .55.45 1 1 1h1c.55 0 1-.45 1-1v-1h8v1c0 .55.45 1 1 1h1c.55 0 1-.45 1-1v-1.78c.61-.55 1-1.34 1-2.22v-3.08c3.39-.49 6-3.39 6-6.92 0-3.87-3.13-7-7-7zM9 6h.29c-.09.32-.16.66-.21.99H3.34C3.89 6.46 5.31 6 9 6zM3 8.99h6.08c.16 1.11.57 2.13 1.18 3.01H3V8.99zM15 18c0 .37-.21.62-.34.73l-.29.27H3.63l-.29-.27C3.21 18.62 3 18.37 3 18v-4h9.41c.78.47 1.65.79 2.59.92V18zm1-5c-2.76 0-5-2.24-5-5s2.24-5 5-5 5 2.24 5 5-2.24 5-5 5zm.5-9H15v5l3.62 2.16.75-1.23-2.87-1.68z"/>
        </SvgIcon>
    }
}
