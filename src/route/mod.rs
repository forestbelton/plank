extern crate router;

use router::Router;

mod new_post;

pub fn router() -> Router {
    let mut router = Router::new();

    router.post("/posts", new_post::handler, "new_post");

    return router;
}