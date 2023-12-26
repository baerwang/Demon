use rand::prelude::SliceRandom;

use crate::common;
use crate::common::{CHROME_VERSIONS, EDGE_VERSIONS, FF_VERSIONS, OS_STRINGS};

pub fn random_user_agent() -> String {
    common::UA_GENS.choose(&mut rand::thread_rng()).unwrap()()
}

pub fn gen_firefox_ua() -> String {
    let version = FF_VERSIONS.choose(&mut rand::thread_rng()).unwrap();
    let os = OS_STRINGS.choose(&mut rand::thread_rng()).unwrap();

    format!(
        "Mozilla/5.0 ({}) Gecko/20100101 Firefox/{}{:.1}",
        os, version, version
    )
}

pub fn gen_chrome_ua() -> String {
    let version = CHROME_VERSIONS.choose(&mut rand::thread_rng()).unwrap();
    let os = OS_STRINGS.choose(&mut rand::thread_rng()).unwrap();
    format!(
        "Mozilla/5.0 ({}) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/{:.1} Safari/537.36",
        os, version
    )
}

pub fn gen_edge_ua() -> String {
    let version = EDGE_VERSIONS.choose(&mut rand::thread_rng()).unwrap();
    let os = OS_STRINGS.choose(&mut rand::thread_rng()).unwrap();
    format!(
        "Mozilla/5.0 ({}) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/{} Safari/537.36 Edg/{}",
        os, version, version
    )
}

#[cfg(test)]
mod tests {
    use common::user_agent;

    use crate::common;

    #[test]
    fn random_user_agent() {
        let buf = std::env::current_dir()
            .unwrap()
            .join("files/user_agent.toml");
        std::env::set_var("user_agent", buf);
        assert_ne!(user_agent::random_user_agent(), "");
    }
}
