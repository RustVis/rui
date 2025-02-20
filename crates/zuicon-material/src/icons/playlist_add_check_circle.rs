// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(PlaylistAddCheckCircle)]
pub fn playlist_add_check_circle(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("PlaylistAddCheckCircle"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M12,2C6.48,2,2,6.48,2,12c0,5.52,4.48,10,10,10c5.52,0,10-4.48,10-10C22,6.48,17.52,2,12,2z M7,7h7v2H7V7z M7,10h7v2H7V10z M10,15H7v-2h3V15z M14.05,18.36l-2.83-2.83l1.41-1.41l1.41,1.41L17.59,12L19,13.41L14.05,18.36z"/>
        </SvgIcon>
    }
}
