pub mod html {
    use handlebars::{Handlebars, RenderError, TemplateError};
    use std::collections::BTreeMap;

    #[derive(Clone)]
    pub struct Renderer {
        hb: Handlebars<'static>,
    }

    impl Renderer {
        const INDEX_T: &'static str = "index";
        const OAUTH_URL_K: &'static str = "OAUTH_URL";

        pub fn new() -> Result<Self, TemplateError> {
            let mut s = Self {
                hb: Handlebars::new(),
            };

            s.hb.register_template_string(Self::INDEX_T, INDEX)?;

            Ok(s)
        }

        pub fn index(&self, oauth_url: &str) -> Result<String, RenderError> {
            let mut data = BTreeMap::new();
            data.insert(Self::OAUTH_URL_K, oauth_url);

            self.hb.render(Self::INDEX_T, &data)
        }
    }

    const INDEX: &'static str = include_str!("./files/index.html");
}

pub mod js {}

pub mod css {}
