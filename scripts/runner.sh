#!/usr/bin/bash

set -eu

cd
if [ ! -e ./problems ]; then
	gsutil -m cp -r gs://icfpc2021/problems ./
fi
mkdir -p ./bin && gsutil -m cp -r gs://icfpc2021/artifacts/215c8fa/* ./bin/ && chmod +x ./bin/*

export START_TIME="$(date +%s)"

: ${PROBLEM_ID:=4}

run() {
	export RUN_ID=$(head -c 1000 /dev/urandom | openssl dgst -md5 -binary | xxd -p | head -c 10)
	export TMPDIR="/tmp/runs/$PROBLEM_ID/$RUN_ID"
	mkdir -p ${TMPDIR}
	{
		timeout 1200s ./bin/wata_rnd < ./problems/$PROBLEM_ID.json > $TMPDIR/wata.json
        CURRENT_TIME="$(date +%s)"
		TIMEOUT="$(( 1200 + START_TIME - CURRENT_TIME ))" \
            timeout "$(( 1300 + START_TIME - CURRENT_TIME ))s" \
            ./bin/chokudai ./problems/$PROBLEM_ID.json $TMPDIR/wata.json >$TMPDIR/chokudai.json
		curl -X POST -d @$TMPDIR/chokudai.json "https://icfpc.sx9.jp/api/submit?problem_id=$PROBLEM_ID"
	} 2>&1 | while read line; do
		echo "$PROBLEM_ID-$RUN_ID: $line"
	done
}

: ${NPROC:=$(nproc)}
: ${JOBS:=$(( NPROC * 2 ))}

for i in $(seq $JOBS); do
    run &
done
wait
