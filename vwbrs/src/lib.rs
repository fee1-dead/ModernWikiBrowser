mod utils;

use wasm_bindgen::prelude::*;

use wiki::deterministic::action::{PageTitleSelector, Parse};
use wiki::deterministic::Main;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn init_wasm() {
    utils::set_panic_hook();
}

#[wasm_bindgen]
pub struct Bot {
    client: wiki::Bot,
}

#[wasm_bindgen]
pub async fn new_bot(oauth_key: String) -> Bot {
    Bot {
        client: wiki::builder::SiteBuilder::enwiki()
            .oauth(oauth_key)
            .build()
            .await
            .unwrap_throw(),
    }
}

#[wasm_bindgen]
impl Bot {
    pub async fn preview(&self, page: String) -> Preview {
        let main = Main::new(Parse::new(PageTitleSelector { page }, |prop| {
            prop.with_text().with_modules()
        }));

        let x = self.client.get_d(main).await.unwrap_throw();
        let text = x.action.parse.text.into_inner();
        let modules = x.action.parse.modulestyles.into_inner().join("|");

        Preview {
            html: text,
            modules,
        }
    }
}

#[wasm_bindgen(getter_with_clone)]
pub struct Preview {
    pub html: String,
    /// list of modules concatenated with `|`.
    pub modules: String,
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, vwbrs!");
}
