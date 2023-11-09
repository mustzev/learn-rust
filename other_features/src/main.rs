mod paas_core;
mod rest_api;
mod try_trait;

fn main() {
    rest_api::rest_func();

    try_trait::try_trait::createTweetAndNotify()
}
