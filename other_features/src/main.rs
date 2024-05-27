mod paas_core;
mod rest_api;
mod try_lifetime;
mod try_smart_pointers;
mod try_trait;

fn main() {
    // rest_api::rest_func();

    // try_trait::try_trait::create_tweet_and_notify();

    // try_lifetime::nested::try_lifetime();

    try_smart_pointers::pointer::make_rc_cons();
}
