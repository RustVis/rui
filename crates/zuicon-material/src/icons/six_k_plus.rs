// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SixKPlus)]
pub fn six_k_plus(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SixKPlus"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M6.5 12.5H8V14H6.5zM19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-9.5 7.5h-3v1h2c.55 0 1 .45 1 1V14c0 .55-.45 1-1 1H6c-.55 0-1-.45-1-1v-4c0-.55.45-1 1-1h3.5v1.5zM16 15h-1.75l-1.75-2.25V15H11V9h1.5v2.25L14.25 9H16l-2.25 3L16 15zm4-2.5h-1.5V14h-1v-1.5H16v-1h1.5V10h1v1.5H20v1z"/>
        </SvgIcon>
    }
}
