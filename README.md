# Parse log data or fixed format data

- You need to write a regular expression in this format
- If you specify some capture in this regular expression, the matched content will be collected and output

## Usage


```shell
stdin | ./parse2csv --regex "^(?<ip>\d{1,3}.\d{1,3}.\d{1,3}.\d{1,3})"
```


## Example

- Prepare data file `cat > access.log`

``` text
94.102.51.144 - - [14/Jun/2024:02:34:17 +0800] "POST /wp-json/wpgmzA/v1/markers?_method=get&random=/wpgmza/v1/markers/10 HTTP/1.1" 301 169 "-" "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3" "-"
91.204.46.40 - - [14/Jun/2024:03:00:42 +0800] "GET /wp-json/wp/v2/users HTTP/1.1" 301 169 "-" "Mozilla/5.0 (Windows NT 6.1; WOW64) AppleWebKit/537.1 (KHTML, like Gecko) Chrome/21.0.1180.83 Safari/537.1" "-"
135.125.246.110 - - [14/Jun/2024:03:13:04 +0800] "GET /.env HTTP/1.1" 404 555 "-" "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/81.0.4044.129 Safari/537.36" "-"
135.125.246.110 - - [14/Jun/2024:03:13:04 +0800] "POST / HTTP/1.1" 405 559 "-" "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/81.0.4044.129 Safari/537.36" "-"
185.254.196.173 - - [14/Jun/2024:03:19:16 +0800] "GET /.env HTTP/1.1" 404 555 "-" "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/81.0.4044.129 Safari/537.36" "-"
```
- Execute

``` shell
cat access.log | ./parse2csv --regex "^(?<ip>\d{1,3}.\d{1,3}.\d{1,3}.\d{1,3}) - - \[(?<date>\S+ \+\d{4})\] \"(?<method>GET|HEAD|POST|PUT|DELETE|CONNECT|OPTIONS|TRACE|PATCH) (?<path>\S+) (?<version>\S+)\" (?<code>\d{3}) (?<rt>\S+) \"(?<referer>\S+)\" \"(?<ua>[^\"]+)\" \"(\S+)\""
```

- Output data in csv format

``` shell
"ip","date","method","path","version","code","rt","referer","ua"
"94.102.51.144","14/Jun/2024:02:34:17 +0800","POST","/wp-json/wpgmzA/v1/markers?_method=get&random=/wpgmza/v1/markers/10","HTTP/1.1","301","169","-","Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3"
"91.204.46.40","14/Jun/2024:03:00:42 +0800","GET","/wp-json/wp/v2/users","HTTP/1.1","301","169","-","Mozilla/5.0 (Windows NT 6.1; WOW64) AppleWebKit/537.1 (KHTML, like Gecko) Chrome/21.0.1180.83 Safari/537.1"
"135.125.246.110","14/Jun/2024:03:13:04 +0800","GET","/.env","HTTP/1.1","404","555","-","Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/81.0.4044.129 Safari/537.36"
"135.125.246.110","14/Jun/2024:03:13:04 +0800","POST","/","HTTP/1.1","405","559","-","Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/81.0.4044.129 Safari/537.36"
"185.254.196.173","14/Jun/2024:03:19:16 +0800","GET","/.env","HTTP/1.1","404","555","-","Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/81.0.4044.129 Safari/537.36"
```

