use warp::{filters::BoxedFilter, Filter};

pub fn path_prefix() -> BoxedFilter<()> {
    warp::any().boxed()
}

pub fn hello() -> BoxedFilter<()> {
    warp::get()
        .and(path_prefix())
        .and(warp::path::end())
        .boxed()
}
