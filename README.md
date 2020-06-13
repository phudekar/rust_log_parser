Log File Parsr
---

This is log file parser return in Rust.

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
