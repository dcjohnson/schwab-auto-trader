pub mod html {
    use handlebars::{Handlebars, RenderError, TemplateError};
    use serde::Serialize;

    #[derive(Serialize)]
    pub struct OauthArgs {
        pub oauth_url: String,
    }

    #[derive(Serialize)]
    pub struct OauthReturnArgs {}

    #[derive(Clone)]
    pub struct Renderer {
        hb: Handlebars<'static>,
    }

    impl Renderer {
        const OAUTH_T: &'static str = "oauth";
        const OAUTH_RETURN_T: &'static str = "oauth_return";
        const FOOTER_P: &'static str = "footer";
        const HEADER_P: &'static str = "header";

        pub fn new() -> Result<Self, TemplateError> {
            let mut s = Self {
                hb: Handlebars::new(),
            };

            s.hb.register_partial(Self::FOOTER_P, FOOTER)?;
            s.hb.register_partial(Self::HEADER_P, HEADER)?;
            s.hb.register_template_string(Self::OAUTH_T, OAUTH)?;
            s.hb.register_template_string(Self::OAUTH_RETURN_T, OAUTH_RETURN)?;

            Ok(s)
        }

        pub fn oauth(&self, args: &OauthArgs) -> Result<String, RenderError> {
            self.hb.render(Self::OAUTH_T, args)
        }

        pub fn oauth_return(&self) -> Result<String, RenderError> {
            self.hb.render(Self::OAUTH_RETURN_T, &OauthReturnArgs {})
        }
    }

    const HEADER: &'static str = include_str!("./files/html/header.html");
    const FOOTER: &'static str = include_str!("./files/html/footer.html");
    const OAUTH: &'static str = include_str!("./files/html/oauth.html");
    const OAUTH_RETURN: &'static str = include_str!("./files/html/oauth_return.html");
}

pub mod js {}

pub mod css {
    pub const MAIN: &'static str = include_str!("./files/css/main.css");
    pub const OAUTH: &'static str = include_str!("./files/css/oauth.css");
}
