# Prometheus Web Exporter

Prometheus Web Exporter is a prometheus exporter that collects information about web pages. You can collect 2 different type of information. Network health info (Response time and response body size) and content based info (Number of elements in a page that matches with given css query.)

Web exporter uses same technology with firefox browser engine (servo) to parse and run css queries. So it is quite fast and supports wide veriety of css queries. Unfortunately it does not run javascript so any dom manipulation done with javascript will not be caught by web exporter. (no SPA support.)

## Configuration

   configuration is done with a file named web_exporter.yaml that is located in the same directory with the executable. Here is an example for web_exporterl.yaml file:

``` yaml
# ip address server will listen. default: 0.0.0.0
# ip_address: "0.0.0.0"
#
# port that servier will listen. default: 3030
# port: 3030
#
# metrics path. default: /metrics
# metrics_path: "metrics"
#
# targets to crawl for each request.
targets:
  # 200 response with queries
  - url: "https://www.rust-lang.org/"
    queries:
      - "#language-values div.flex-none section"
      - "header h1"
      - "footer div.attribution"
    extra_labels:
      name: homepage
  # 404 response with queries
  - url: "https://www.rust-lang.org/invalid-page-with-404-response"
    headers:
      Referer: "https://www.rust-lang.org/"
    queries:
      - "div.flex"
      - "div"
    extra_labels:
      name: 404 page
  # Network error. (Queries will not return any value since they will not be running.)
  - url: "https://www.page-does-not-exist.io/"
    queries:
      - "div"
    extra_labels:
      name: nonexistent_page
  # Invalid query (return value will be 0 and css query parse error will be logged.)
  - url: "https://www.rust-lang.org/invalid-css-query"
    queries:
      - "**XX**"
    extra_labels:
      name: query_with_invalid_css
  # 200 page without any query (only response time and size will be returned.)
  - url: "https://www.rust-lang.org/no-css-query"
  # google search test example with queryparameters and extra headers.
  - url: "https://www.google.com/search"
    queryparameters:
      q: rust
    headers:
      referer: "https://www.google.com/"
      user-agent: "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_4) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/83.0.4103.61 Safari/537.36"
    queries:
      - "div.g"
    extra_labels:
      name: google_search
```

Configuration above will generate metrics like following:

``` txt
web_exporter_response_duration_milliseconds{url="https://www.rust-lang.org/", method="GET", error="false", name="homepage", status="200" } 787
web_exporter_response_response_size_bytes{url="https://www.rust-lang.org/", method="GET", error="false", name="homepage", status="200" } 19220
web_exporter_query_count{url="https://www.rust-lang.org/", method="GET", error="false", name="homepage", status="200", query="#language-values div.flex-none section" } 3
web_exporter_query_count{url="https://www.rust-lang.org/", method="GET", error="false", name="homepage", status="200", query="header h1" } 1
web_exporter_query_count{url="https://www.rust-lang.org/", method="GET", error="false", name="homepage", status="200", query="footer div.attribution" } 1
web_exporter_response_duration_milliseconds{error="false", url="https://www.rust-lang.org/invalid-page-with-404-response", status="404", name="404 page", method="GET" } 142
web_exporter_response_response_size_bytes{error="false", url="https://www.rust-lang.org/invalid-page-with-404-response", status="404", name="404 page", method="GET" } 8244
web_exporter_query_count{error="false", url="https://www.rust-lang.org/invalid-page-with-404-response", status="404", name="404 page", method="GET", query="div.flex" } 6
web_exporter_query_count{error="false", url="https://www.rust-lang.org/invalid-page-with-404-response", status="404", name="404 page", method="GET", query="div" } 14
web_exporter_response_duration_milliseconds{name="nonexistent_page", status="0", url="https://www.page-does-not-exist.io/", method="GET", error="true" } 83
web_exporter_response_response_size_bytes{name="nonexistent_page", status="0", url="https://www.page-does-not-exist.io/", method="GET", error="true" } 0
web_exporter_response_duration_milliseconds{error="false", name="query_with_invalid_css", method="GET", status="404", url="https://www.rust-lang.org/invalid-css-query" } 110
web_exporter_response_response_size_bytes{error="false", name="query_with_invalid_css", method="GET", status="404", url="https://www.rust-lang.org/invalid-css-query" } 8244
web_exporter_query_count{error="false", name="query_with_invalid_css", method="GET", status="404", url="https://www.rust-lang.org/invalid-css-query", query="**XX**" } 0
web_exporter_response_duration_milliseconds{error="false", status="404", url="https://www.rust-lang.org/no-css-query", method="GET" } 127
web_exporter_response_response_size_bytes{error="false", status="404", url="https://www.rust-lang.org/no-css-query", method="GET" } 8244
web_exporter_response_duration_milliseconds{name="google_search", url="https://www.google.com/search", status="200", method="GET", error="false" } 964
web_exporter_response_response_size_bytes{name="google_search", url="https://www.google.com/search", status="200", method="GET", error="false" } 406579
web_exporter_query_count{name="google_search", url="https://www.google.com/search", status="200", method="GET", error="false", query="div.g" } 11
web_exporter_scrape_duration_milliseconds 972
```
## How to get it
### With docker.
``` bash
# Download sample configuration from github
wget https://raw.githubusercontent.com/huseyinyilmaz/web_exporter/master/sample_web_exporter.yaml
# Rename it to web_exporter.yaml
mv sample_web_exporter.yaml web_exporter.yaml
# Run Docker and mount configuration file to container
$ docker run \
       --rm -ti -d \
       -p 3030:3030 \
       --name web_exporter \
       -v $(pwd)/web_exporter.yaml:/usr/local/prometheus_web_exporter/web_exporter.yaml \
       huseyinyilmaz/web_exporter:v1.0.5
# endpoint should be on http://localhost:3030/metrics
```
### With docker-compose

First download sample configuration from repository and put it in the same directory as docker-compose.yaml file. Then you can add following config to your docker-compose file.
```
  webexporter:
    image: huseyinyilmaz/web_exporter:v1.0.5
    volumes:
      - ./web_exporter.yaml:/usr/local/prometheus_web_exporter/web_exporter.yaml
    environment:
      WEB_EXPORTER_LOG_LEVEL: warn
    ports:
      - 3030:3030
    expose:
      - 3030
```

### With Cargo

``` bash
# First install web_exporter with cargo.
cargo install prometheus_web_exporter
# After installation cargo will show where it is installed it is likely on
# ~/.cargo/bin. Go to installation folder and download sample config next to executable.
cd ~/.cargo/bin
# Download sample configuration from github
wget https://raw.githubusercontent.com/huseyinyilmaz/web_exporter/master/sample_web_exporter.yaml
# Rename it to web_exporter.yaml
mv sample_web_exporter.yaml web_exporter.yaml
# run web_exporter
./prometheus_web_exporter
# endpoint should be on http://localhost:3030/metrics
```

### With prebuilt binary.
You can download the binary for your os from releases section in github. After getting the binary, just put the configuration next to binary you are good to go.

## FAQ
### How can I integrate it to prometheus:

``` yaml
  - job_name: 'webexporter'
    scrape_interval: 60s
    scrape_timeout: 50s
    # metrics_path defaults to '/metrics'
    # scheme defaults to 'http'.
    static_configs:
    - targets: ['webexporter:3030']
```
## How can I change scrape frequency.

Web scraper does not have any internal cache and it will run the scrape every time /metrics endpoint is called. Scrape frequency can be changed by changing `scrape_interval` value from premetheus configuration.
``` yaml
  - job_name: 'webexporter'
    scrape_interval: 30s # <- Change scrape frequency from here.
    scrape_timeout: 29s
    # metrics_path defaults to '/metrics'
    # scheme defaults to 'http'.
    static_configs:
    - targets: ['webexporter:3030']
```
## How can I change log level.
Logging level can be set via environment variables. To run the project with info logging level you can run it like this

``` bash
$ WEB_EXPORTER_LOG_LEVEL=info ./prometheus_web_exporter
```

If you are using docker image, log level is already set to WARN.

## One of the metrics return with label "error": "true" what does that mean?
It means website you are scraping cannot complete http request and it is returning network error. Those errors will also be logged to stdout by the exporter. If you are using docker container you should see the errors on docker logs.
