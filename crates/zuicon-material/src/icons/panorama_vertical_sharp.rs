// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(PanoramaVerticalSharp)]
pub fn panorama_vertical_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("PanoramaVerticalSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M17.46 4c-.77 2.6-1.16 5.28-1.16 8 0 2.72.39 5.41 1.16 8H6.55c.77-2.6 1.16-5.28 1.16-8 0-2.72-.39-5.41-1.16-8h10.91m2.78-2H3.77s.26.77.3.88C5.16 5.82 5.71 8.91 5.71 12s-.55 6.18-1.64 9.12c-.04.11-.3.88-.3.88h16.47s-.26-.77-.3-.88c-1.09-2.94-1.64-6.03-1.64-9.12s.55-6.18 1.64-9.12c.04-.11.3-.88.3-.88z"/>
        </SvgIcon>
    }
}
