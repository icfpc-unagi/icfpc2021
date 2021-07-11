#!/usr/bin/bash

set -eu

cd
if [ ! -e ./problems ]; then
	gsutil -m cp -r gs://icfpc2021/problems ./
fi
mkdir -p ./bin && gsutil -m cp -r gs://icfpc2021/artifacts/f372a48/* ./bin/ && chmod +x ./bin/*

: ${PROBLEM_ID:=4}

run() {
	export RUN_ID=$(head -c 1000 /dev/urandom | openssl dgst -md5 -binary | xxd -p | head -c 10)
	export TMPDIR="/tmp/runs/$PROBLEM_ID/$RUN_ID"
	mkdir -p ${TMPDIR}
	{
		./bin/wata < ./problems/$PROBLEM_ID.json > $TMPDIR/wata.json
		./bin/chokudai ./problems/$PROBLEM_ID.json $TMPDIR/wata.json >$TMPDIR/chokudai.json
		curl -X POST -d @$TMPDIR/chokudai.json "https://icfpc.sx9.jp/api/submit?problem_id=$PROBLEM_ID"
	} 2>&1 | while read line; do
		echo "$PROBLEM_ID-$RUN_ID: $line"
	done
}

: ${NPROC:=$(nproc)}
: ${JOBS:=$(( NPROC * 5 / 4 ))}

for i in $(seq $JOBS); do
    run &
done
wait
