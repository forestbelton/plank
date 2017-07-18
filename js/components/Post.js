import moment from 'moment';
import React from 'react';
import ReactDOM from 'react-dom';
import PropTypes from 'prop-types';

import './Post.css';

const Post = (props) => {
    const date = moment(props.create_date).format('[on] DD MMM YYYY [at] HH:mm:ss');

    return (<div className="Post">
        Posted by <div className="Post-Author">{props.author}</div>
        &nbsp;<div className="Post-CreateDate">{date}</div>
        <div className="Post-ID">{props.uuid}</div>
        <div className="Post-Content">
            {props.children}
        </div>
    </div>);
};

Post.propTypes = {
    uuid: PropTypes.string.isRequired,
    author: PropTypes.string.isRequired,
    create_date: PropTypes.string.isRequired,
    attachment: PropTypes.string
};

export default Post;