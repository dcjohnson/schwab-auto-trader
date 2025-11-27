pub mod html {
    use handlebars::{Handlebars, RenderError, TemplateError};
    use serde::Serialize;

    #[derive(Serialize)]
    pub struct OauthArgs {
        pub oauth_url: String,
    }

    #[derive(Serialize)]
    pub struct OauthReturnArgs {
        pub oauth_return_message: String,
    }

    #[derive(Serialize)]
    pub struct Root {
        pub account_value: f64,
    }

    #[derive(Clone)]
    pub struct Renderer {
        hb: Handlebars<'static>,
    }

    impl Renderer {
        const ROOT_T: &'static str = "root";
        const OAUTH_T: &'static str = "oauth";
        const OAUTH_RETURN_T: &'static str = "oauth_return";
        const FOOTER_P: &'static str = "footer";
        const HEADER_P: &'static str = "header";

        pub fn new() -> Result<Self, TemplateError> {
            let mut s = Self {
                hb: Handlebars::new(),
            };

            s.hb.register_partial(Self::ROOT_T, ROOT)?;
            s.hb.register_partial(Self::FOOTER_P, FOOTER)?;
            s.hb.register_partial(Self::HEADER_P, HEADER)?;
            s.hb.register_template_string(Self::OAUTH_T, OAUTH)?;
            s.hb.register_template_string(Self::OAUTH_RETURN_T, OAUTH_RETURN)?;

            Ok(s)
        }

        pub fn root(&self, args: &Root) -> Result<String, RenderError> {
            self.hb.render(Self::ROOT_T, args)
        }

        pub fn oauth(&self, args: &OauthArgs) -> Result<String, RenderError> {
            self.hb.render(Self::OAUTH_T, args)
        }

        pub fn oauth_return(&self, args: &OauthReturnArgs) -> Result<String, RenderError> {
            self.hb.render(Self::OAUTH_RETURN_T, args)
        }
    }

    const HEADER: &'static str = include_str!("./files/html/header.html");
    const FOOTER: &'static str = include_str!("./files/html/footer.html");
    const ROOT: &'static str = include_str!("./files/html/root.html");
    const OAUTH: &'static str = include_str!("./files/html/oauth.html");
    const OAUTH_RETURN: &'static str = include_str!("./files/html/oauth_return.html");
}

pub mod js {}

pub mod css {
    pub const ROOT: &'static str = include_str!("./files/css/root.css");
    pub const HEADER: &'static str = include_str!("./files/css/header.css");
    pub const OAUTH: &'static str = include_str!("./files/css/oauth.css");
    pub const OAUTH_RETURN: &'static str = include_str!("./files/css/oauth_return.css");
}
