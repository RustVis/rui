// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(PlaylistAddCircleOutlined)]
pub fn playlist_add_circle_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("PlaylistAddCircleOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M12,2C6.48,2,2,6.48,2,12s4.48,10,10,10s10-4.48,10-10S17.52,2,12,2z M12,20c-4.41,0-8-3.59-8-8s3.59-8,8-8s8,3.59,8,8 S16.41,20,12,20z M14,10H7v2h7V10z M14,7H7v2h7V7z M7,15h3v-2H7V15z M19,13v2h-2v2h-2v-2h-2v-2h2v-2h2v2H19z"/>
        </SvgIcon>
    }
}
