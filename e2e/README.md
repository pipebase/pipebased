pipebased e2e test workspace
## Local Development
install pipebuilder mock server
```sh
cargo install pipebuilder_mock --bin mock
```
clone pre-built app repository
```sh
git clone https://github.com/pipebase/repository.git PATH/TO/REPOSITORY
```
setup local data volume
```sh
# at project root
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
create default app user/group
```sh
sudo groupadd pipebase --gid 10000 && \
    sudo useradd pipebase --uid 10000 --gid 10000
```
run daemon
```sh
# at project root
sudo RUST_LOG=info PIPEBASED_LOG_FORMATTER=full PIPEBASED_CONFIG_FILE=e2e/resources/piped.yml ./target/release/piped
```

## Test Sample App
install cli
```sh
cargo install --path pipebased --bin pipe
```
pull app
```sh
pipe pulla -n dev -i timer -v 0
```
pull catalogs
```sh
pipe pullc -n dev -i timer -v 0
```
list app
```sh
pipe apps
```
list catalogs
```sh
pipe catalogs
```
create pipe
```sh
# at project root
pipe create e2e/resources/pipe-compose.yml
```
start pipe
```sh
pipe start timer
```
check pipe runtime state
```sh
pipe ps
```
stop pipe
```sh
pipe stop timer
```
remove pipe
```sh
pipe rm timer
```
remove app
```sh
pipe rma -n dev -i timer -v 0
```
remove catalogs
```sh
pipe rmc -n dev -i timer -v 0
```
