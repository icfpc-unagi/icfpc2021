package handler

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"github.com/golang/glog"
	"github.com/imos/icfpc2021/internal/api"
	"github.com/pkg/errors"
	"net/http"
	"os"
	"os/exec"
	"strconv"
)

func init() {
	http.HandleFunc("/estimate_score", handleEstimateScore)
}

func handleEstimateScore(w http.ResponseWriter, r *http.Request) {
	ctx := r.Context()

	if r.Method == "GET" {
		fmt.Fprintln(w, `
<body><form action="?" method="POST">
Problem ID: <input type="text" name="problem_id"><br><br>
JSON:
<textarea name="data"></textarea><br><br>
<input type="submit" value="Submit">
</form></body>
`)
		return
	}
	r.ParseForm()
	problemID, err := strconv.ParseInt(r.Form.Get("problem_id"), 10, 64)
	if err != nil {
		fmt.Fprintf(w, "Failed to parse problem_id: %+v", err)
	}
	evaluation, err := EstimateScore(ctx, problemID, r.Form.Get("data"))
	if err != nil {
		fmt.Fprintf(w, "Failed to submit: %+v", err)
	}
	fmt.Fprintf(w, "Evaluation: %v", evaluation)
}

func EstimateScore(ctx context.Context, problemID int64, solution string) (*api.Evaluation, error) {
	cmd := exec.CommandContext(ctx, "bash", "-c",
		fmt.Sprintf("evaluate /problems/%d.json <(cat)", problemID))
	cmd.Stdin = bytes.NewBuffer([]byte(solution))
	cmd.Stderr = os.Stderr
	output, _ := cmd.Output()
	glog.Infof("Evaluation: %s", string(output))
	evaluation := &api.Evaluation{}
	if err := json.Unmarshal(output, evaluation); err != nil {
		return nil, errors.Wrapf(err, "failed to parse output")
	}
	return evaluation, nil
}
