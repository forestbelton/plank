import PropTypes from 'prop-types';
import React from 'react';
import ReactDOM from 'react-dom';

import Post from './Post';

const OriginalPost = props => {
    const replies = props.replies.map(reply =>
        <Post key={reply.uuid} {...reply}>
            {reply.body}
        </Post>
    );

    return (
    <Post className="OriginalPost" {...props}>
        {props.body}
        <div className="Post-Replies">
            {replies}
        </div>
    </Post>);
};

OriginalPost.propTypes = Object.assign(
    {},
    Post.propTypes,
    {
        replies: PropTypes.arrayOf(PropTypes.shape(Post.propTypes)).isRequired
    }
);

export default OriginalPost;