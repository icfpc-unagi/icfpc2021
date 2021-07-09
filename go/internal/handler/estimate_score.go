package handler

import (
	"bytes"
	"context"
	"fmt"
	"github.com/pkg/errors"
	"net/http"
	"os"
	"os/exec"
	"strconv"
	"strings"
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
	submissionID, err := EstimateScore(ctx, problemID, r.Form.Get("data"))
	if err != nil {
		fmt.Fprintf(w, "Failed to submit: %+v", err)
	}
	fmt.Fprintf(w, "Score: %d", submissionID)
}

func EstimateScore(ctx context.Context, problemID int64, solution string) (int64, error) {
	cmd := exec.CommandContext(ctx, "bash", "-c",
		fmt.Sprintf("calculate_score /problems/%d.json <(cat)", problemID))
	cmd.Stdin = bytes.NewBuffer([]byte(solution))
	cmd.Stderr = os.Stderr
	output, _ := cmd.Output()
	result := strings.TrimSpace(string(output))
	score, err := strconv.ParseInt(result, 10, 64)
	if err != nil {
		return 0, errors.Wrapf(err, "failed to parse output: %s", result)
	}
	return score, nil
}
