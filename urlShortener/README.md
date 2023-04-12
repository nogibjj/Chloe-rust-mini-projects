# Actix-Web URL Shortener Example

This project is a URL shortener web application built with Actix-Web.

## Usage

Start the web server by running the following command:

```
cargo run
make run
```

Once the server is running, you can access the application at http://localhost:8080.


### Shortening URLs

To shorten a URL, send a POST request to the /shorten endpoint with a url parameter containing the URL you want to shorten:

```
curl -X POST -d 'url=http://example.com' http://localhost:8080/shorten
```

The response will be the shortened URL, for example:

```
http://localhost:8080/F6TI7o
```

### Redirecting Shortened URLs

To redirect a shortened URL to its original URL, simply visit the shortened URL in your web browser.

## Implementation Details

The application uses a HashMap to store the original URLs and their corresponding shortened URLs. The HashMap is wrapped in a Mutex and stored in the application state to ensure thread-safety.

When a URL is shortened, a random alphanumeric slug is generated and associated with the original URL in the HashMap.

When a shortened URL is visited, the slug is extracted from the URL path and used to look up the original URL in the HashMap. If the original URL is found, a Found response is returned with a Location header set to the original URL. If the original URL is not found, a 404 Not Found error is returned.

