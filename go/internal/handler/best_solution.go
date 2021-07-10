package handler

import (
	"fmt"
	"github.com/golang/glog"
	"github.com/imos/icfpc2021/pkg/db"
	"net/http"
	"strconv"
)

func init() {
	http.HandleFunc("/best_solution", handleBestSolution)
}

func handleBestSolution(w http.ResponseWriter, r *http.Request) {
	ctx := r.Context()
	problemID, err := strconv.ParseInt(r.URL.Query().Get("problem_id"), 10, 64)
	if err != nil {
		w.WriteHeader(404)
		return
	}
	result, err := db.CellString(ctx, `
SELECT submission_data FROM submissions NATURAL JOIN (
    SELECT MIN(submission_id) AS submission_id FROM submissions NATURAL JOIN (
        SELECT problem_id, MIN(submission_score) AS submission_score
        FROM submissions WHERE problem_id = ? AND submission_score >= 0
		GROUP BY problem_id
    ) r
) r
`, problemID)
	if err != nil {
		glog.Errorf("Failed to query: %+v", err)
		w.WriteHeader(500)
		return
	}
	fmt.Fprintf(w, result)
}
