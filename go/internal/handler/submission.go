package handler

import (
	"fmt"
	"net/http"
	"strconv"

	"github.com/golang/glog"
	"github.com/imos/icfpc2021/pkg/db"
)

func init() {
	http.HandleFunc("/submission", handleSubmission)
}

func handleSubmission(w http.ResponseWriter, r *http.Request) {
	ctx := r.Context()
	query := r.URL.Query()
	submissionID, err := strconv.ParseInt(query.Get("submission_id"), 10, 64)
	if err != nil {
		w.WriteHeader(404)
		return
	}
	var result string
	result, err = db.CellString(ctx, `
SELECT submission_data FROM submissions WHERE submission_id = ? 
`, submissionID)
	if err != nil {
		glog.Errorf("Failed to query: %+v", err)
		w.WriteHeader(500)
		return
	}
	fmt.Fprintf(w, result)
}
