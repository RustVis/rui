// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use yew::{function_component, html, Html};
use zu::svg_icon::{Color, FontSize, Props, SvgIcon};
use zuicon_material::Home;

#[function_component(Abc)]
pub fn abc(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            view_box={props.view_box.clone()}
            >
                <path d="M21,11h-1.5v-0.5h-2v3h2V13H21v1c0,0.55-0.45,1-1,1h-3c-0.55,0-1-0.45-1-1v-4c0-0.55,0.45-1,1-1h3c0.55,0,1,0.45,1,1V11z M8,10v5H6.5v-1.5h-2V15H3v-5c0-0.55,0.45-1,1-1h3C7.55,9,8,9.45,8,10z M6.5,10.5h-2V12h2V10.5z M13.5,12c0.55,0,1,0.45,1,1v1 c0,0.55-0.45,1-1,1h-4V9h4c0.55,0,1,0.45,1,1v1C14.5,11.55,14.05,12,13.5,12z M11,10.5v0.75h2V10.5H11z M13,12.75h-2v0.75h2V12.75z"/>
        </SvgIcon>
    }
}

#[function_component(IconsPage)]
pub fn icons_page() -> Html {
    html! {
        <div class="container">
        <h1>{"Icons"}</h1>

        <h2>{"SvgIcon"}</h2>
        <p>{"If you need a custom SVG icon you can use the SvgIcon wrapper.\
            This component extends the native <svg> element:"}</p>
        <div class="demo-box">
            <Home />

            <SvgIcon>
                <path d="M10 20v-6h4v6h5v-8h3L12 3 2 12h3v8z" />
            </SvgIcon>
        </div>

        <h3>{"Color"}</h3>
        <div class="demo-box">
            <Home />
            <Home color={Color::Primary} />
            <Home color={Color::Secondary} />
            <Home color={Color::Success} />
            <Home color={Color::Action} />
            <Home color={Color::Disabled} />
            <Home style="color: var(--zu-colors-pink-500);" />
        </div>

        <Abc color={Color::Error} font_size={FontSize::Large} />

        </div>
    }
}
