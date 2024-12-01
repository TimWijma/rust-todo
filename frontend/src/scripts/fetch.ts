export class Fetch {
    static async get(url: string, params: any = {}) {
        let apiUrl = Fetch.createUrl(url, params);
        return fetch(apiUrl, {
            method: "GET",
        }).then(Fetch.responseHandler);
    }

    static async post(url: string, body: any = {}, params: any = {}) {
        let apiUrl = Fetch.createUrl(url, params);

        const requestBody = typeof body === "string" ? body : JSON.stringify(body);

        return fetch(apiUrl, {
            method: "POST",
            body: requestBody,
            headers: {
                "Content-Type": "application/json",
            },
        }).then(Fetch.responseHandler);
    }

    static async put(url: string, body: any = {}, params: any = {}) {
        let apiUrl = Fetch.createUrl(url, params);

        const requestBody = typeof body === "string" ? body : JSON.stringify(body);

        return fetch(apiUrl, {
            method: "PUT",
            body: requestBody,
        }).then(Fetch.responseHandler);
    }

    static async delete(url: string, params: any = {}) {
        let apiUrl = Fetch.createUrl(url, params);
        return fetch(apiUrl, {
            method: "DELETE",
        }).then(Fetch.responseHandler);
    }

    private static async responseHandler(response: Response) {
        return response.text().then((text) => {
            if (!response.ok) {
                let error = {
                    status: response.status,
                    statusText: response.statusText,
                    message: "",
                    url: response.url,
                };

                if (text) {
                    try {
                        error.message = JSON.parse(text);
                    } catch (e) {
                        error.message = text;
                    }
                }

                return Promise.reject(error);
            }
            return text && JSON.parse(text);
        });
    }

    private static createUrl(url: string, params: any = {}) {
        let apiUrl = new URL(url);
        apiUrl.search = new URLSearchParams(params).toString();
        return apiUrl;
    }
}
