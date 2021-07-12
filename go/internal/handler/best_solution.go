package handler

import (
	"fmt"
	"net/http"
	"strconv"

	"github.com/golang/glog"
	"github.com/imos/icfpc2021/pkg/db"
)

func init() {
	http.HandleFunc("/best_solution", handleBestSolution)
}

func handleBestSolution(w http.ResponseWriter, r *http.Request) {
	ctx := r.Context()
	query := r.URL.Query()
	problemID, err := strconv.ParseInt(query.Get("problem_id"), 10, 64)
	obtained_bonus := query.Get("obtained_bonus")
	required_bonus := query.Get("required_bonus")
	if err != nil {
		w.WriteHeader(404)
		return
	}
	var result string
	conds := "problem_id = ?"
	params := []interface{}{problemID}
	if len(obtained_bonus) > 0 {
		conds += " AND INSTR(submission_obtained_bonuses, ?) > 0"
		params = append(params, obtained_bonus)
	}
	if len(required_bonus) > 0 {
		conds += " AND INSTR(submission_bonuses, ?) > 0"
		params = append(params, required_bonus)
	} else {
		conds += " AND submission_bonuses = ''"
	}
	result, err = db.CellString(ctx, `
	SELECT submission_data FROM submissions NATURAL JOIN (
			SELECT MIN(submission_id) AS submission_id FROM submissions NATURAL JOIN (
					SELECT problem_id, MIN(submission_estimated_score) AS submission_estimated_score
					FROM submissions WHERE `+conds+` AND submission_estimated_score >= 0
			GROUP BY problem_id
			) r
	) r
	`, params...)
	if err != nil {
		glog.Errorf("Failed to query: %+v", err)
		w.WriteHeader(500)
		return
	}
	fmt.Fprintf(w, result)
}
