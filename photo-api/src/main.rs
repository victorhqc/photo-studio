#[macro_use]
extern crate gotham_derive;

use dotenv::dotenv;

fn main() {
    #[cfg(target_os = "linux")]
    openssl_probe::init_ssl_cert_env_vars();

    dotenv().ok();
    pretty_env_logger::init();
}
