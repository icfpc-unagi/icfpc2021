package handler

import (
	"context"
	"encoding/json"
	"github.com/golang/glog"
	"github.com/google/uuid"
	"github.com/imos/icfpc2021/internal/api"
	"github.com/imos/icfpc2021/pkg/db"
	"github.com/pkg/errors"
	"io/ioutil"
	"net/http"
)

func init() {
	http.HandleFunc("/api/run/acquire", handleRunAcquire)
	http.HandleFunc("/api/run/extend", handleRunExtend)
	http.HandleFunc("/api/run/flush", handleRunFlush)
}

func handleRunAcquire(w http.ResponseWriter, r *http.Request) {
	ctx := r.Context()
	resp, err := doRunAcquire(ctx)
	if err != nil {
		glog.Errorf("Failed to do run_acquire: %+v", err)
		w.WriteHeader(500)
		return
	}
	buf, err := json.Marshal(resp)
	if err != nil {
		glog.Errorf("Failed to marshal a response: %+v", err)
		w.WriteHeader(500)
		return
	}
	if _, err := w.Write(buf); err != nil {
		glog.Errorf("Failed to write buffer: %+v", err)
		w.WriteHeader(500)
		return
	}
}

func doRunAcquire(ctx context.Context) (*api.RunAcquireResponse, error) {
	var resp api.RunAcquireResponse
	signature := uuid.New().String()
	result, err := db.Execute(ctx, `
UPDATE runs
SET
	run_signature = ?,
	run_locked = CURRENT_TIMESTAMP() + INTERVAL 1 MINUTE 
WHERE run_locked < CURRENT_TIMESTAMP()
ORDER BY run_locked LIMIT 1
`, signature)
	if err != nil {
		return nil, errors.Wrapf(err, "failed to run an SQL command")
	}
	n, err := result.RowsAffected()
	if err != nil {
		return nil, errors.Wrapf(err, "failed to get # of affected rows")
	}
	if n == 0 {
		return &resp, nil
	}
	if err := db.Row(ctx, &resp, `
SELECT run_id, run_command, run_signature
FROM runs WHERE run_signature = ?
LIMIT 1
`,
	signature); err != nil {
		return nil, errors.Wrapf(err, "")
	}
	return &resp, nil
}


func handleRunExtend(w http.ResponseWriter, r *http.Request) {
	ctx := r.Context()
	if r.Body == nil {
		w.WriteHeader(400)
		return
	}
	defer r.Body.Close()
	buf, err := ioutil.ReadAll(r.Body)
	if err != nil {
		w.WriteHeader(500)
		return
	}
	var req api.RunExtendRequest
	if err := json.Unmarshal(buf, &req); err != nil {
		glog.Errorf("Failed to parse a rqeuest: %+v", req)
		w.WriteHeader(400)
		return
	}
	err = doRunExtend(ctx, &req)
	if err != nil {
		glog.Errorf("Failed to extend the lock: %+v", err)
		w.WriteHeader(500)
		return
	}
}

func doRunExtend(ctx context.Context, req *api.RunExtendRequest) error {
	result, err := db.Execute(ctx, `
UPDATE runs
SET run_locked = CURRENT_TIMESTAMP() + INTERVAL 1 MINUTE
WHERE run_signature = ? AND CURRENT_TIMESTAMP() < run_locked
LIMIT 1
`, req.RunSignature)
	if err != nil {
		return errors.Wrapf(err, "failed to extend the lock")
	}
	n, err := result.RowsAffected()
	if err != nil {
		return errors.Wrapf(err, "failed to get # of rows affected")
	}
	if n == 0 {
		return errors.New("failed to extend the lock")
	}
	return nil
}

func handleRunFlush(w http.ResponseWriter, r *http.Request) {
	ctx := r.Context()
	if r.Body == nil {
		w.WriteHeader(400)
		return
	}
	defer r.Body.Close()
	buf, err := ioutil.ReadAll(r.Body)
	if err != nil {
		w.WriteHeader(500)
		return
	}
	var req api.RunFlushRequest
	if err := json.Unmarshal(buf, &req); err != nil {
		glog.Errorf("Failed to parse a rqeuest: %+v", req)
		w.WriteHeader(400)
		return
	}
	err = doRunFlush(ctx, &req)
	if err != nil {
		glog.Errorf("Failed to extend the lock: %+v", err)
		w.WriteHeader(500)
		return
	}
}

func doRunFlush(ctx context.Context, req *api.RunFlushRequest) error {
	result, err := db.Execute(ctx, `
UPDATE runs
SET
	run_locked = CURRENT_TIMESTAMP() + INTERVAL 10 YEAR,
	run_signature = "",
	run_code = ?,
	run_stdout = ?,
	run_stderr = ?
WHERE run_signature = ?
LIMIT 1
`,
req.RunCode,
req.RunStdout,
req.RunStderr,
req.RunSignature)
	if err != nil {
		return errors.Wrapf(err, "failed to flush")
	}
	n, err := result.RowsAffected()
	if err != nil {
		return errors.Wrapf(err, "failed to flush")
	}
	if n == 0 {
		return errors.New("no run to flush")
	}
	return nil
}
