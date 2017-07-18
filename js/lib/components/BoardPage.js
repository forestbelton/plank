import PropTypes from 'prop-types';
import React from 'react';
import ReactDOM from 'react-dom';

import Api from '../api/Api';
import FetchPosts from '../api/actions/FetchPosts';

import OriginalPost from '../components/post/OriginalPost';

class BoardPage extends React.Component {

    constructor(props) {
        super(props);
        this.state = { loading: true };
    }

    componentDidMount() {
        FetchPosts(this.props.api)
            .then(posts => this.setState({
                loading: false,
                posts
            }));
    }

    render() {
        if (this.state.loading) {
            return <span>Loading...</span>;
        }

        const posts = this.state.posts.map(post =>
            <OriginalPost key={post.uuid} {...post} />);

        return (
        <div className="BoardPage">
            {posts}
        </div>);
    }
}

BoardPage.propTypes = {
    api: PropTypes.instanceOf(Api).isRequired
};

export default BoardPage;