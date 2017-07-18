import Api from './Api';

class HttpApi extends Api {
    constructor(baseURL) {
        super();
        this.baseURL = baseURL;
    }

    get(path) {
        const url = this.baseURL + path;

        return fetch(url)
            .then(resp => resp.json());
    }
}

export default HttpApi;