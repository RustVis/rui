// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(AudioFile)]
pub fn audio_file(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("AudioFile"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M14,2H6C4.9,2,4.01,2.9,4.01,4L4,20c0,1.1,0.89,2,1.99,2H18c1.1,0,2-0.9,2-2V8L14,2z M16,13h-3v3.75 c0,1.24-1.01,2.25-2.25,2.25S8.5,17.99,8.5,16.75c0-1.24,1.01-2.25,2.25-2.25c0.46,0,0.89,0.14,1.25,0.38V11h4V13z M13,9V3.5 L18.5,9H13z"/>
        </SvgIcon>
    }
}
