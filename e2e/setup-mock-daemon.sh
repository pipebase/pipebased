#!/usr/bin/env bash
set -o nounset
set -o pipefail

# Entrypoint: main

# workspace
workspace="e2e"
resources="resources"
mock="mock.yml"
daemon="piped.yml"
sleep_period=5

function usage() { 
cat <<EOF
Checker
Options:
	-d | --directory
	path to workspace directory (default: e2e)
	-h | --help
	print usage
Usage:
	$0 -d </PATH/TO/WORKSPACE>
EOF
exit 1
}

function parse_args() {
	while [[ $# -gt 0 ]]
	do
		i="$1"
		case ${i} in
			-d|--directory)
			if [ $# -lt 2 ]; then
				usage
			fi
			workspace="$2"
			shift
			shift
			;;
			-h|--help)
			usage
			shift
			shift
			;;
			*)
			usage
			;;
		esac
	done
	if [ ! -d "${workspace}" ]; then
		echo "Directory '${workspace}' not found, exit ..." 1>&2;
		exit 1;
	fi
    resources="${workspace}/${resources}"
    mock="${resources}/${mock}"
    if [ ! -f "${mock}" ]; then
        echo "Mock config file '${mock} not found, exit ...'" 1>&2
        exit 1;
    fi
    daemon="${resources}/${daemon}"
    if [ ! -f "${daemon}" ]; then
        echo "Daemon config file '${daemon} not found, exit ...'" 1>&2
        exit 1;
    fi
}

function run_mock() {
    RUST_LOG=info PIPEBUILDER_CONFIG_FILE=${mock} mock &
}

function run_all() {
    run_mock
    # sleep ${sleep_period}
}

# Entrypoint to setup daemon and pipebuilder mock server
function main() {
    parse_args $@
    run_all
}

main $@
