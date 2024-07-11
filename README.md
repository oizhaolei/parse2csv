# リリース

- access log parser
- Read the squid | zscaler access log file, parse it line by line, and export the data to a new CSV file.

## 実行方法

```shell
./access_log_parser --input ./out/xxxxx.mail.com --regex "^(?<ip>\d{1,3}.\d{1,3}.\d{1,3}.\d{1,3}) - - \[(?<date>\S+ \+\d{4})\] \"(?<method>GET|HEAD|POST|PUT|DELETE|CONNECT|OPTIONS|TRACE|PATCH) (?<path>\S+) (?<version>\S+)\" (?<code>\d{3}) (?<rt>\S+) \"(?<referer>\S+)\" \"(?<ua>[^\"]+)\" \"(\S+)\""
```
new csv file should named with `./out/xxxxx.mail.com.csv`


## デモ

``` shell
$ cat access.log

94.102.51.144 - - [14/Jun/2024:02:34:17 +0800] "POST /wp-json/wpgmzA/v1/markers?_method=get&random=/wpgmza/v1/markers/10 HTTP/1.1" 301 169 "-" "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3" "-"
91.204.46.40 - - [14/Jun/2024:03:00:42 +0800] "GET /wp-json/wp/v2/users HTTP/1.1" 301 169 "-" "Mozilla/5.0 (Windows NT 6.1; WOW64) AppleWebKit/537.1 (KHTML, like Gecko) Chrome/21.0.1180.83 Safari/537.1" "-"
135.125.246.110 - - [14/Jun/2024:03:13:04 +0800] "GET /.env HTTP/1.1" 404 555 "-" "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/81.0.4044.129 Safari/537.36" "-"
135.125.246.110 - - [14/Jun/2024:03:13:04 +0800] "POST / HTTP/1.1" 405 559 "-" "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/81.0.4044.129 Safari/537.36" "-"
185.254.196.173 - - [14/Jun/2024:03:19:16 +0800] "GET /.env HTTP/1.1" 404 555 "-" "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/81.0.4044.129 Safari/537.36" "-"

### 実行

$ ./access_log_parser --input ./demo/access.log --regex "^(?<ip>\d{1,3}.\d{1,3}.\d{1,3}.\d{1,3}) - - \[(?<date>\S+ \+\d{4})\] \"(?<method>GET|HEAD|POST|PUT|DELETE|CONNECT|OPTIONS|TRACE|PATCH) (?<path>\S+) (?<version>\S+)\" (?<code>\d{3}) (?<rt>\S+) \"(?<referer>\S+)\" \"(?<ua>[^\"]+)\" \"(\S+)\""

$ cat demo/access.log.csv 
"ip","date","method","path","version","code","rt","referer","ua"
"94.102.51.144","14/Jun/2024:02:34:17 +0800","POST","/wp-json/wpgmzA/v1/markers?_method=get&random=/wpgmza/v1/markers/10","HTTP/1.1","301","169","-","Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3"
"91.204.46.40","14/Jun/2024:03:00:42 +0800","GET","/wp-json/wp/v2/users","HTTP/1.1","301","169","-","Mozilla/5.0 (Windows NT 6.1; WOW64) AppleWebKit/537.1 (KHTML, like Gecko) Chrome/21.0.1180.83 Safari/537.1"
"135.125.246.110","14/Jun/2024:03:13:04 +0800","GET","/.env","HTTP/1.1","404","555","-","Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/81.0.4044.129 Safari/537.36"
"135.125.246.110","14/Jun/2024:03:13:04 +0800","POST","/","HTTP/1.1","405","559","-","Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/81.0.4044.129 Safari/537.36"
"185.254.196.173","14/Jun/2024:03:19:16 +0800","GET","/.env","HTTP/1.1","404","555","-","Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/81.0.4044.129 Safari/537.36"
```

