# Prometheus Web Exporter

Prometheus Web Exporter is a prometheus exporter that collects information about web pages. You can collect 2 different type of information from url. Network health info (response time and response body size) and content based info (Number of elements in a page that matches with given css query.)

Web exporter uses same technology with firefox browser engine (servo) to parse and run css queries. So it is quite fast and supports wide veriety of css queries. Unfortunately it does not run javascript so any dom manipulation done with javascript will not be caught by web exporter.

## Configuration
   Web configuration is done with a file named web_exporter.yaml that is located in the same directory with the executable. Here is an example for web_exporterl.yaml file:

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
  # 404 response with queries
  - url: "https://www.rust-lang.org/invalid-page-with-404-response"
    queries:
      - "div.flex"
      - "div"
  # Network error. (Queries will not return any value since they will not be running.)
  - url: "https://www.page-does-not-exist.io/"
    queries:
      - "div"
  # Invalid query (return value will be 0 and css query parse error will be logged.)
  - url: "https://www.rust-lang.org/invalid-css-query"
    queries:
      - "**XX**"
  # 200 page without any query (only response time and size will be returned.)
  - url: "https://www.rust-lang.org/no-css-query"
```

Configuration above will generate metrics like following:

``` txt
web_exporter_response_duration_milliseconds{url="https://www.rust-lang.org/", status=200, error=0} 640
web_exporter_response_response_size_bytes{url="https://www.rust-lang.org/", status=200, error=0} 19220
web_exporter_query_count{url="https://www.rust-lang.org/", query="#language-values div.flex-none section", status=200, error=0} 3
web_exporter_query_count{url="https://www.rust-lang.org/", query="header h1", status=200, error=0} 1
web_exporter_query_count{url="https://www.rust-lang.org/", query="footer div.attribution", status=200, error=0} 1
web_exporter_response_duration_milliseconds{url="https://www.rust-lang.org/invalid-page-with-404-response", status=404, error=0} 292
web_exporter_response_response_size_bytes{url="https://www.rust-lang.org/invalid-page-with-404-response", status=404, error=0} 8244
web_exporter_query_count{url="https://www.rust-lang.org/invalid-page-with-404-response", query="div.flex", status=404, error=0} 6
web_exporter_query_count{url="https://www.rust-lang.org/invalid-page-with-404-response", query="div", status=404, error=0} 14
web_exporter_response_duration_milliseconds{url="https://www.page-does-not-exist.io/", status=0, error=1} 270
web_exporter_response_response_size_bytes{url="https://www.page-does-not-exist.io/", status=0, error=1} 0
web_exporter_response_duration_milliseconds{url="https://www.rust-lang.org/invalid-css-query", status=404, error=0} 306
web_exporter_response_response_size_bytes{url="https://www.rust-lang.org/invalid-css-query", status=404, error=0} 8244
web_exporter_query_count{url="https://www.rust-lang.org/invalid-css-query", query="**XX**", status=404, error=0} 0
web_exporter_scrape_duration_milliseconds 641
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
       huseyinyilmaz/web_exporter:v1.0.0
# endpoint should be on http://localhost:3030/metrics
```
### With docker-compose

First download sample configuration from repository and put it in the same directory as docker-compose.yaml file. Then you can add following config to your docker-compose file.
```
  webexporter:
    image: huseyinyilmaz/web_exporter:v1.0.1
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
## Logging:
Logging configuration can be provided through environment variables. To run the project with info logging level you can run it like this

``` bash
$ WEB_EXPORTER_LOG_LEVEL=info ./prometheus_web_exporter
```
