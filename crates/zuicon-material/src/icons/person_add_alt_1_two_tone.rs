// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(PersonAddAlt1TwoTone)]
pub fn person_add_alt_1_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("PersonAddAlt1TwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M9,16c-2.7,0-5.8,1.29-6,2h12C14.78,17.28,11.69,16,9,16z" opacity=".3"/><path d="M9,14c-2.67,0-8,1.34-8,4v2h16v-2C17,15.34,11.67,14,9,14z M3,18c0.2-0.71,3.3-2,6-2c2.69,0,5.78,1.28,6,2H3z"/><path d="M9,12c2.21,0,4-1.79,4-4c0-2.21-1.79-4-4-4S5,5.79,5,8C5,10.21,6.79,12,9,12z M9,6c1.1,0,2,0.9,2,2c0,1.1-0.9,2-2,2 S7,9.1,7,8C7,6.9,7.9,6,9,6z"/>
        </SvgIcon>
    }
}
