// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(LibraryAddCheckSharp)]
pub fn library_add_check_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("LibraryAddCheckSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M22,2H6v16h16V2z M12.47,14L9,10.5l1.4-1.41l2.07,2.08L17.6,6L19,7.41L12.47,14z M4,6H2v16h16v-2H4V6z"/><path d="M0,0h24v24H0V0z" fill="none"/>
        </SvgIcon>
    }
}
