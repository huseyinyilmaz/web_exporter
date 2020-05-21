# Prometheus Web Exporter

Prometheus Web Exporter collects information about status of web pages from internet.

It send a request to a webpage and runs one or more css queries on the result. Number of results for each query becomes a metric on premetheus.

## Configuration
   Web configuration is done with a file named web_exporter.yaml with the same directory with the executable here is an example fonfiguration for web_exporterl.yaml fie

``` yaml
ip_address: "0.0.0.0"
port: 3030
targets:
  - url: "https://www.rust-lang.org/"
    queries:
      - "#language-values div.flex-none section"
      - "header h1"
      - "footer div.attribution"

  - url: "https://www.rust-lang.org/invalid-page-with-404-response"
    queries:
      - "div.flex"
      - "div"

  - url: "https://www.page-does-not-exist.io/"
    queries:
      - "div"

  - url: "https://www.rust-lang.org/invalid-css-query"
    queries:
      - "**XX**"
```

Configuration above will generate metrics like following:

``` txt
web_exporter_query{url="https://www.rust-lang.org/", query="#language-values div.flex-none section", status=200, error=0} 3
web_exporter_query{url="https://www.rust-lang.org/", query="header h1", status=200, error=0} 1
web_exporter_query{url="https://www.rust-lang.org/", query="footer div.attribution", status=200, error=0} 1
web_exporter_query{url="https://www.rust-lang.org/invalid-page-with-404-response", query="div.flex", status=404, error=0} 6
web_exporter_query{url="https://www.rust-lang.org/invalid-page-with-404-response", query="div", status=404, error=0} 14
web_exporter_query{url="https://www.page-does-not-exist.io/", query="div", status=0, error=1} 0
web_exporter_query{url="https://www.rust-lang.org/invalid-css-query", query="**XX**", status=0, error=1} 0
web_exporter_scrape_duration_milliseconds 386
```

## Logging:
Logging configuration can be provided through environment variables. To run the project with info logging level you can run it like this

``` bash
$ WEB_EXPORTER_LOG_LEVEL=info ./prometheus_web_exporter
```

## RUN WITH DOCKER

```
docker run -p 3030:3030 --name web_exporter -v /Users/huseyin/projects/web_exporter/web_exporter.yaml:/home/prometheus_web_exporter/bin/web_exporter.yaml -e WEB_EXPORTER_LOG_LEVEL=info huseyinyilmaz/web_exporter:v0.1.6
```