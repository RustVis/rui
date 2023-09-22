// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Bing {}

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

impl Component for Bing {
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
                data-icon={ "bing" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M8.35 5.046a.615.615 0 0 0-.54.575c-.009.13-.006.14.289.899.67 1.727.833 2.142.86 2.2.067.142.16.276.277.395.089.092.148.141.247.208.176.117.262.15.944.351.664.197 1.026.327 1.338.482.405.201.688.43.866.7.128.195.242.544.291.896.02.137.02.44 0 .564-.041.27-.124.495-.252.684-.067.1-.044.084.055-.039.278-.346.562-.938.707-1.475a4.42 4.42 0 0 0-2.14-5.028 69.556 69.556 0 0 0-.888-.465 50.419 50.419 0 0 0-.53-.277l-.353-.184c-.16-.082-.266-.138-.345-.18-.368-.192-.523-.27-.568-.283a.93.93 0 0 0-.194-.03l-.063.007Z"/>
  <path d="M9.152 11.493a2.623 2.623 0 0 0-.135.083 320.256 320.256 0 0 0-1.513.934 164.12 164.12 0 0 0-.8.496c-.012.01-.587.367-.876.543a1.91 1.91 0 0 1-.732.257c-.12.017-.349.017-.47 0a1.891 1.891 0 0 1-.884-.358 2.45 2.45 0 0 1-.365-.364 1.884 1.884 0 0 1-.34-.76 1.441 1.441 0 0 0-.027-.121c-.005-.006.004.092.022.22.018.132.057.324.098.489a4.096 4.096 0 0 0 2.487 2.796c.359.142.72.23 1.114.275.147.016.566.023.72.011a4.103 4.103 0 0 0 1.956-.661l.235-.149a36.314 36.314 0 0 0 .394-.248l.258-.163 1.164-.736c.51-.32.663-.433.9-.665.099-.097.248-.262.255-.283.002-.005.028-.046.059-.091a1.64 1.64 0 0 0 .25-.682c.02-.124.02-.427 0-.565a2.875 2.875 0 0 0-.213-.758c-.15-.314-.47-.6-.928-.83a1.986 1.986 0 0 0-.273-.12c-.006 0-.433.26-.948.58a1699.479 1699.479 0 0 0-1.113.687l-.295.183Z"/>
  <path d="m3.004 12.184.03.129c.089.402.245.693.515.963a1.823 1.823 0 0 0 1.312.543c.361 0 .673-.09.994-.287l.472-.29.373-.23V5.334c0-1.537-.003-2.45-.008-2.521a1.817 1.817 0 0 0-.535-1.177c-.097-.096-.18-.16-.427-.33A1192.515 1192.515 0 0 1 4.183.24c-.239-.163-.258-.175-.33-.2a.631.631 0 0 0-.842.464c-.009.042-.01.603-.01 3.646l.003 8.035Z"/>
            </svg>
        }
    }
}
