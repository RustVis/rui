// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(PanoramaHorizontalSharp)]
pub fn panorama_horizontal_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("PanoramaHorizontalSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M4 6.55c2.6.77 5.28 1.16 8 1.16 2.72 0 5.41-.39 8-1.16v10.91c-2.6-.77-5.28-1.16-8-1.16-2.72 0-5.41.39-8 1.16V6.55M2 3.77v16.47s.77-.26.88-.3C5.82 18.85 8.91 18.3 12 18.3c3.09 0 6.18.55 9.12 1.64.11.04.88.3.88.3V3.77s-.77.26-.88.3C18.18 5.15 15.09 5.71 12 5.71s-6.18-.56-9.12-1.64c-.11-.05-.88-.3-.88-.3z"/>
        </SvgIcon>
    }
}
