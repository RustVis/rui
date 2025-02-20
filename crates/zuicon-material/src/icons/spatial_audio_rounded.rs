// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SpatialAudioRounded)]
pub fn spatial_audio_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SpatialAudioRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M22.11,7.95c-1.89-0.23-5.57-1.83-6.09-6.09C15.96,1.36,15.54,1,15.04,1h0c-0.6,0-1.07,0.53-1,1.13 c0.31,2.43,2.38,7.12,7.8,7.8c0.6,0.08,1.13-0.4,1.13-1v0C22.97,8.43,22.6,8.01,22.11,7.95z"/><path d="M21.71,5.83c0.64,0.17,1.26-0.31,1.26-0.97c0-0.47-0.34-0.85-0.79-0.97c-0.49-0.14-1.72-0.68-2.11-2.13 C19.95,1.32,19.57,1,19.11,1H19.1c-0.66,0-1.14,0.64-0.96,1.28C18.74,4.5,20.58,5.53,21.71,5.83z"/><path d="M16.39,15.56C14.71,14.7,12.53,14,10,14c-2.53,0-4.71,0.7-6.39,1.56C2.61,16.07,2,17.1,2,18.22V21h16v-2.78 C18,17.1,17.39,16.07,16.39,15.56z"/>
        </SvgIcon>
    }
}
