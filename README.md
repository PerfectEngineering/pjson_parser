# A Rust JSON Parser
> Not to be used in production, it is purely for learning rust and how its library system works. you can through the codebase to learn a few things too.

This is a JSON Parser implementation. Its primary function is to parse a JSON string into an Intermediate Token Structure or deserialized into a struct (this part is yet to be implemented).

Also, it can handle parsing bignumber's because it parses Numbers into strings (which can later be deserialized/coerced into whatever type the struct defines for that field).

Currently one possible drawback is that the parsing is implemented using recursion, which mean it is not ideal for deeply nested JSON object parsing, other than this, it is fine.