// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(InsertChartOutlinedSharp)]
pub fn insert_chart_outlined_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("InsertChartOutlinedSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M9 17H7v-7h2v7zm4 0h-2V7h2v10zm4 0h-2v-4h2v4zm2 2H5V5h14v14zm2-16H3v18h18V3z"/>
        </SvgIcon>
    }
}
