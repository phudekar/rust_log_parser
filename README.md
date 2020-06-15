Log File Parser
---
!["build-status"](https://travis-ci.org/phudekar/rust_log_parser.svg?branch=master&status=unknown)

This is a log file parser return in Rust which tries to parse lines in a file as log messages. Once the parsing is done it then sorts the messages according to timestamp and prints out all the errors with given severity.

This code parses log files which have logs in following format.

`[message_type] [timestamp] [message]`

Any message which does not follow above give structure will be classified as `UnknownMessage`.

`message_type` could be one of the following

- `I` : Info
- `W` : Warning
- `E [error_code]` : Error

For example:

>`"E 2 562 help help"` 

would be parse into

>`LogMessage (Error 2) 562 "help help"`

and 

>`"I 29 la la la"`

would result in 

>`LogMessage Info 29 "la la la"`
---
## How to run program to get error messages

> `cargo run <file_name> [severity]`

example without severity
>`cargo run abc.log`

example with severity
>`cargo run abc.log 20`

**Note**: Severity is optional. If no value is provided then severity is taken as `50`
