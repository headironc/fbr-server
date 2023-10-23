use async_once_cell::OnceCell;

use crate::config::Config;

static STATE: OnceCell<State> = OnceCell::new();

pub struct State {
    config: &'static Config,
}

impl State {
    pub async fn instance() -> &'static State {
        STATE
            .get_or_init(async {
                let config = Config::instance();

                State { config }
            })
            .await
    }

    pub fn config(&self) -> &'static Config {
        self.config
    }
}
