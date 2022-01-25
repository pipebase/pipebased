#!/usr/bin/env bash
set -o nounset
set -o pipefail

# Entrypoint: main

# workspace
workspace="e2e"
# data directory
data_directory=""
# repository
repository=""
# repository app
repository_app=""
# repository catalogs
repository_catalogs=""
# target platform
target_platform="x86_64-unknown-linux-gnu"

function usage() { 
cat <<EOF
Checker
Options:
	-d | --directory
	path to workspace directory (default: e2e)
    -r | --repository
    path to artifact repository
    -t | --target
    target platform
	-h | --help
	print usage
Usage:
	$0 -d </PATH/TO/WORKSPACE> -r </PATH/TO/REPOSITORY>
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
            -r|--repository)
			if [ $# -lt 2 ]; then
				usage
			fi
			repository="$2"
			shift
			shift
			;;
            -t|--target)
			if [ $# -lt 2 ]; then
				usage
			fi
			target_platform="$2"
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
		echo "Directory ${workspace} not found, exit ..." 1>&2;
		exit 1;
	fi
    if [ ! -d "${repository}" ]; then
		echo "Repository ${repository} not found, exit ..." 1>&2;
		exit 1;
	fi
    if [ ! -d "${repository}/binary/${target_platform}/app" ]; then
        echo "binary for target ${target_platform} not found, exit ..." 1>&2
        exit 1;
    fi
    repository_app="${repository}/binary/${target_platform}/app"
    repository_catalogs="${repository}/catalogs"
    data_directory="${workspace}/data"
}

function setup() {
    # setup daemon data volume
    mkdir -p ${data_directory}/daemon/app
    mkdir -p ${data_directory}/daemon/catalogs
    mkdir -p ${data_directory}/daemon/workspace
    # setup mock server data volume
    mkdir -p ${data_directory}/mock
    ln -s ${repository_app} ${data_directory}/mock/app
    ln -s ${repository_catalogs} ${data_directory}/mock/catalogs
}

function cleanup() {
    rm -rf ${data_directory}
}

# Entrypoint of data volume setup script
function main() {
    parse_args $@
    cleanup
    setup
}

main $@
