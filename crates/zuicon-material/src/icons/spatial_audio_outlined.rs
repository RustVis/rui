// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SpatialAudioOutlined)]
pub fn spatial_audio_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SpatialAudioOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M10,13c2.21,0,4-1.79,4-4c0-2.21-1.79-4-4-4S6,6.79,6,9C6,11.21,7.79,13,10,13z M10,7c1.1,0,2,0.9,2,2c0,1.1-0.9,2-2,2 s-2-0.9-2-2C8,7.9,8.9,7,10,7z"/><path d="M16.39,15.56C14.71,14.7,12.53,14,10,14c-2.53,0-4.71,0.7-6.39,1.56C2.61,16.07,2,17.1,2,18.22V21h16v-2.78 C18,17.1,17.39,16.07,16.39,15.56z M16,19H4v-0.78c0-0.38,0.2-0.72,0.52-0.88C5.71,16.73,7.63,16,10,16 c2.37,0,4.29,0.73,5.48,1.34C15.8,17.5,16,17.84,16,18.22V19z"/><path d="M16,1h-2c0,4.97,4.03,9,9,9V8C19.14,8,16,4.86,16,1z"/><path d="M20,1h-2c0,2.76,2.24,5,5,5V4C21.35,4,20,2.65,20,1z"/>
        </SvgIcon>
    }
}
