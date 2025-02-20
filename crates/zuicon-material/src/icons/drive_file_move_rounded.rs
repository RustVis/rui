// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(DriveFileMoveRounded)]
pub fn drive_file_move_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("DriveFileMoveRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M20,6h-8l-1.41-1.41C10.21,4.21,9.7,4,9.17,4H4C2.9,4,2,4.9,2,6v12c0,1.1,0.9,2,2,2h16c1.1,0,2-0.9,2-2V8 C22,6.9,21.1,6,20,6z M12,15.79V14H9c-0.55,0-1-0.45-1-1v0c0-0.55,0.45-1,1-1h3v-1.79c0-0.45,0.54-0.67,0.85-0.35l2.79,2.79 c0.2,0.2,0.2,0.51,0,0.71l-2.79,2.79C12.54,16.46,12,16.24,12,15.79z"/>
        </SvgIcon>
    }
}
