// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(NotesTwoTone)]
pub fn notes_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("NotesTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M21 11.01L3 11v2h18zM3 16h12v2H3zM21 6H3v2.01L21 8z"/>
        </SvgIcon>
    }
}
