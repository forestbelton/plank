extern crate router;

use router::Router;

mod get_posts;
mod new_post;

pub fn router() -> Router {
    let mut router = Router::new();

    router.get("/posts", get_posts::handler, "get_posts");
    router.post("/posts", new_post::handler, "new_post");

    return router;
}