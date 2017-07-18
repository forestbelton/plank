import { parse, format } from 'date-fns';
import React from 'react';
import ReactDOM from 'react-dom';
import PropTypes from 'prop-types';

import './Post.css';

const Post = (props) => {
    const className = props.className ? `Post ${props.className}` : 'Post';
    const date = format(parse(props.create_date), '[on] DD MMM YYYY [at] HH:mm:ss');

    return (
    <div className={className}>
        Posted by <div className="Post-Author">{props.author}</div>
        &nbsp;<div className="Post-CreateDate">{date}</div>
        <div className="Post-ID">{props.uuid}</div>
        <div className="Post-Content">
            {props.children}
        </div>
    </div>);
};

Post.propTypes = {
    className: PropTypes.string,
    uuid: PropTypes.string.isRequired,
    author: PropTypes.string.isRequired,
    create_date: PropTypes.string.isRequired,
    attachment: PropTypes.string
};

export default Post;