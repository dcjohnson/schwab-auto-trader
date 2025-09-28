pub mod html {
    use handlebars::{Handlebars, RenderError, TemplateError};
    use std::collections::BTreeMap;

    #[derive(Clone)]
    pub struct Renderer {
        hb: Handlebars<'static>,
    }

    impl Renderer {
        const INDEX_T: &'static str = "index";
        const FOOTER_P: &'static str = "footer";
        const HEADER_P: &'static str = "header";

        const OAUTH_URL_K: &'static str = "OAUTH_URL";

        pub fn new() -> Result<Self, TemplateError> {
            let mut s = Self {
                hb: Handlebars::new(),
            };

            s.hb.register_partial(Self::FOOTER_P, FOOTER)?;
            s.hb.register_partial(Self::HEADER_P, HEADER)?;
            s.hb.register_template_string(Self::INDEX_T, INDEX)?;

            Ok(s)
        }

        pub fn index(&self, oauth_url: &str) -> Result<String, RenderError> {
            let mut data = BTreeMap::new();
            data.insert(Self::OAUTH_URL_K, oauth_url);

            self.hb.render(Self::INDEX_T, &data)
        }
    }

    const HEADER: &'static str = include_str!("./files/html/header.html");
    const FOOTER: &'static str = include_str!("./files/html/footer.html");
    const INDEX: &'static str = include_str!("./files/html/index.html");
}

pub mod js {
    pub const MAIN: &'static str = include_str!("./files/js/main.js");
}

pub mod css {
    pub const MAIN: &'static str = include_str!("./files/css/main.css");
    pub const OAUTH: &'static str = include_str!("./files/css/oauth.css");
}
