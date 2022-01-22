pipebased e2e test workspace
## Local Development
install pipebuilder mock server
```sh
cargo install pipebuilder_mock --bin mock
```
setup local data volume
```
./e2e/setup-data-volume.sh -r PATH/TO/REPOSITORY
```
build daemon
```sh
cargo build --package pipebased --release
```
run pipebuilder mock server
```sh
# at project root
RUST_LOG=info PIPEBUILDER_LOG_FORMATTER=full PIPEBUILDER_CONFIG_FILE=e2e/resources/mock.yml mock
```
run daemon
```sh
# at project root
sudo RUST_LOG=info PIPEBASED_LOG_FORMATTER=full PIPEBASED_CONFIG_FILE=e2e/resources/piped.yml ./target/release/piped
```

## Test Sample App