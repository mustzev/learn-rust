mod paas_core;
mod rest_api;
mod try_concurrency;
mod try_lifetime;
mod try_oop_features;
mod try_patterns_and_matching;
mod try_smart_pointers;
mod try_trait;

fn main() {
    // rest_api::rest_func();

    // try_trait::try_trait::create_tweet_and_notify();

    // try_lifetime::nested::try_lifetime();

    // try_smart_pointers::pointer::make_rc_cons();

    // try_smart_pointers::interior_mutability::make_interior_mutability();

    // try_smart_pointers::reference_cycle::make_reference_cycle();

    // try_concurrency::threads::make_threads();

    // try_concurrency::threads::pass_message_between_threads();

    // try_concurrency::mutex::make_use_of_mutex();

    // try_concurrency::mutex::make_use_of_mutex_1();

    try_patterns_and_matching::refutability::make_destructuring_nested_structs_and_enums();
}
