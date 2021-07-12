#!/usr/bin/bash

set -eu

: ${PROBLEM_ID:=4}
: ${NPROC:=$(nproc)}
: ${JOBS:=$(( NPROC * 4 ))}
: ${COMMIT_ID:=e68bc48}

cd
mkdir -p ./problems ./bin
gsutil cp gs://icfpc2021/problems/$PROBLEM_ID.json ./problems/
for program in chokudai wata_rnd; do
    gsutil -m cp -r gs://icfpc2021/artifacts/$COMMIT_ID/$program ./bin/
done
chmod +x ./bin/*

export START_TIME="$(date +%s)"

run() {
	export RUN_ID=$(head -c 1000 /dev/urandom | openssl dgst -md5 -binary | xxd -p | head -c 10)
	export TMPDIR="/tmp/runs/$PROBLEM_ID/$RUN_ID"
	mkdir -p ${TMPDIR}
	{
		timeout 1200s ./bin/wata_rnd < ./problems/$PROBLEM_ID.json > $TMPDIR/wata.json
        CURRENT_TIME="$(date +%s)"
        DEADLINE="$(( 1800 + (RANDOM % JOBS) / 10 ))"
		TIMEOUT="$(( 1800 + START_TIME - CURRENT_TIME ))" \
            timeout "$(( 2000 + START_TIME - CURRENT_TIME ))s" \
            ./bin/chokudai ./problems/$PROBLEM_ID.json $TMPDIR/wata.json >$TMPDIR/chokudai.json
		curl -X POST -d @$TMPDIR/chokudai.json "https://icfpc.sx9.jp/api/submit?problem_id=$PROBLEM_ID"
	} 2>&1 | while read line; do
		echo "$PROBLEM_ID-$RUN_ID: $line"
	done
}


for i in $(seq $JOBS); do
    run &
done
wait
