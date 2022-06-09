# fanboy_exporter

Prometheus exporter for [FanBoy](https://github.com/lynix/fanboy)

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

![Example](https://github.com/lynix/fanboy_exporter/blob/master/grafana.png)


## Usage

```
$ fanboy_exporter [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -b <ADDR>        Listen address (default: 0.0.0.0)
    -d <DEV>         Device (default: /dev/ttyACM0)
    -i <INT>         Update interval in seconds (default: 10)
    -p <PORT>        TCP port (default: 9184)
```

## Bugs / Features

Pull requests are always welcome. Feel free to report bugs or post questions
using the *issues* function on GitHub.


## License

This project is published under the terms of the *MIT License*. See the file
`LICENSE` for more information.
