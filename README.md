# リリース

- access log parser

## 実行方法

### Command Line

```shell
[PIPELINE STREAM] | ./access_log_parser --inputfile ./out/xxxxx.mail.com --regex "^(?<ip>\d{1,3}.\d{1,3}.\d{1,3}.\d{1,3}) - - \[(?<date>\S+ \+\d{4})\] \"(?<method>GET|HEAD|POST|PUT|DELETE|CONNECT|OPTIONS|TRACE|PATCH) (?<path>\S+) (?<version>\S+)\" (?<code>\d{3}) (?<rt>\S+) \"(?<referer>\S+)\" \"(?<ua>[^\"]+)\" \"(\S+)\""
```
