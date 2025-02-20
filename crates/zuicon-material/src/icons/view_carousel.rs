// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ViewCarousel)]
pub fn view_carousel(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ViewCarousel"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M2,7h4v10H2V7z M7,19h10V5H7V19z M18,7h4v10h-4V7z"/>
        </SvgIcon>
    }
}
