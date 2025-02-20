// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct KeubernetesLogo {}

#[derive(Properties, Debug, Clone, PartialEq, Eq)]
pub struct Props {
    #[prop_or_default]
    pub class: Option<&'static str>,

    #[prop_or_default]
    pub width: Option<&'static str>,

    #[prop_or_default]
    pub height: Option<&'static str>,

    #[prop_or_default]
    pub color: Option<&'static str>,

    #[prop_or_default]
    pub fill: Option<&'static str>,

    #[prop_or_default]
    pub spin: bool,

    #[prop_or_default]
    pub rotate: i16,
}

impl Component for KeubernetesLogo {
    type Properties = Props;
    type Message = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        // TODO(Shaohua): Do not generate style attribute if it is empty.
        let mut style = String::new();
        if props.rotate != 0 {
            style += &format!("transform: rotate({}deg);", props.rotate);
        }
        html! {
            <svg
                xmlns={ "http://www.w3.org/2000/svg" }
                class={ props.class.unwrap_or("") }
                width={ props.width.unwrap_or("16") }
                height={ props.height.unwrap_or("16") }
                focusable={ "false" }
                data-icon={ "KeubernetesLogo" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M793 1075v1l-215 62q-3-24-3-48 0-61 16-119t49-111l168 150v1q13 11 13 28 0 12-8 22t-20 14zm164 65l-16-66 42-52h68l42 53-15 65-61 29-60-29zm-66-234h-1L707 776q54-52 118-82t139-41l-13 224h-1q-1 15-11 25t-26 11q-14 0-22-7zm159 409h1l109 196q-35 10-69 16t-73 7q-73 0-142-24l108-195q5-8 13-14t19-6q11 0 20 5t14 15zm-183-73q0 8-3 15l1 1-86 206q-61-39-106-96t-71-126q7-1 25-4t41-8 48-8 48-8 38-6 21-3q17 0 30 8t14 29zm345-38l222 37q-25 69-70 127t-108 97q-3-7-11-25t-17-41-21-50-19-48-15-39-6-21q0-16 11-26t27-11h7zm827 96q0 48-29 85l-492 612q-20 24-47 37t-59 14H623q-31 0-58-13t-48-38L24 1385q-14-18-21-40t-8-45q0-14 3-29l176-762q7-29 26-54t47-38L958 77q28-13 58-13 15 0 30 3t30 10l711 340q28 14 47 37t27 54l175 763q3 15 3 29zm-251-41q0-26-20-36t-48-14-57-5-46-9q-6-3-10-8t-9-11l-18-5q2-20 3-39t2-40q0-84-24-165t-72-150q5-3 8-6t8-8q0-21 21-36t48-30 48-31 22-41q0-18-12-29t-30-12q-23 0-40 16t-35 37-36 36-43 17h-3l-16 12q-72-75-166-120t-197-55q-1-5-1-9t0-10q-11-10-13-16t-3-20q0-31 6-60t6-60q0-19-12-34t-32-15q-20 0-31 15t-12 35q0 29 5 57t6 57q0 14-2 22t-13 19l-1 18q-104 8-198 54T605 702l-8-6-8-6q-8 2-13 2-15 0-32-16t-35-37-37-37-41-17q-17 0-28 12t-12 29q0 24 21 40t48 31 48 29 22 39l15 12q-47 70-71 150t-25 165q0 40 6 80l-18 5q-4 5-8 11t-11 8q-17 7-46 8t-56 5-48 14-21 37q0 19 14 30t32 11q18 0 35-6t35-14 34-14 36-7q8 0 13 2t12 7l19-3q32 100 97 180t155 135l-8 19q5 10 5 19 0 13-11 32t-26 39-25 40-12 35q0 17 11 30t30 13q14 0 25-8t17-20h1q3-10 9-29t14-39 14-38 14-24q7-7 19-9l10-18q48 19 99 28t103 9q51 0 102-9t100-27l9 16q7 2 12 4t11 10q10 15 17 41t17 51 22 43 35 19q18 0 29-13t12-31q0-15-11-35t-26-40-26-39-12-34q0-5 1-8t4-8l-7-18q91-53 155-135t97-182q5 1 9 2t10 1q5-3 9-6t10-3q19 0 37 6t36 14 36 14 37 7q17 0 31-11t14-30zm-704-382h-1l-12-224q72 9 137 40t118 83l-182 130-1-1q-8 7-22 7t-25-10-12-25zm309-17q32 52 49 111t17 122v22q0 11-2 22l-216-62v-1q-12-3-19-13t-8-22q0-19 13-30l166-149z" />
            </svg>
        }
    }
}
