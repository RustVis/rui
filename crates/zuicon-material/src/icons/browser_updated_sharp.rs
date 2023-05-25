// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(BrowserUpdatedSharp)]
pub fn browser_updated_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("BrowserUpdatedSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M22,13v5h-5l1,1v2H6v-2l1-1H2V3l10,0v2L4,5v11h16v-3H22z M15,15l-5-5h4V3h2v7h4L15,15z"/>
        </SvgIcon>
    }
}
