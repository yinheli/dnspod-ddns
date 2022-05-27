# dnspod ddns

Simple dnspod ddns tool.

## install

```bash
cargo install dnspod-ddns
```

or download from github release https://github.com/yinheli/dnspod-ddns/releases

## usage

```bash
dnspod-ddns --help
```

```
USAGE:
    dnspod-ddns [OPTIONS] --domain <DOMAIN> --sub-domain <SUB_DOMAIN> --token <TOKEN>

OPTIONS:
    -d, --domain <DOMAIN>            domain, eg: home.example.com
    -h, --help                       Print help information
    -i, --interval <INTERVAL>        check interval seconds, eg: 10 default: 10 min: 5 max: 3600
                                     [env: DNSPOD_INTERVAL=] [default: 10]
    -s, --sub-domain <SUB_DOMAIN>    sub domain, eg: www
    -t, --token <TOKEN>              dnspod api key / token, eg:
                                     12345,aeaae98e8fbee8369f93ec46c4384aed [env: DNSPOD_API_KEY=]
    -V, --version                    Print version information
        --verbose                    verbose log
```

## resource

- dnspod documentation https://docs.dnspod.cn/api/record-list/
