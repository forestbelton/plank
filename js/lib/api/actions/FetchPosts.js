// TODO: optimize
const normalizePosts = posts => {
    const parentPosts = posts.filter(post => post.reply_id === null);

    return parentPosts.map(post => {
        post.replies = posts.filter(reply => reply.reply_id === post.id);
        return post;
    });
};

const FetchPosts = api => api
    .get('/posts')
    .then(normalizePosts);

export default FetchPosts;