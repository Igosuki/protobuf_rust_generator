### Open RTB Model Generator

Generate rust source files containing structs and enums for the OpenRTB specification protobuf schema.

#### Running example

```
MY_PROTO_FILE=realtime-bidding.proto
RUST_BACKTRACE=1 cargo run -- --files $(pwd)/$MY_PROTO_FILE --out-dir target/generated
```

N.B. : protoc must be installed and available on the path. 
