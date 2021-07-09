package handler

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"github.com/golang/glog"
	"github.com/imos/icfpc2021/pkg/db"
	"github.com/pkg/errors"
	"io/ioutil"
	"net/http"
	"os"
	"strconv"
)

func init() {
	http.HandleFunc("/api/submit", handleSubmit)
}

func handleSubmit(w http.ResponseWriter, r *http.Request) {
	ctx := r.Context()

	if r.Body == nil {
		glog.Errorf("body is empty")
		w.WriteHeader(400)
		return
	}
	defer r.Body.Close()
	buf, err := ioutil.ReadAll(r.Body)
	if err != nil {
		glog.Errorf("body is broken: %#v", err)
		w.WriteHeader(400)
		return
	}

	problemID, err := strconv.ParseInt(
		r.URL.Query().Get("problem_id"), 10, 64)
	if err != nil {
		glog.Errorf("Failed to parse problem ID: %+v", err)
		w.WriteHeader(400)
		return
	}

	poseID, err := Submit(ctx, problemID, string(buf))
	if err != nil {
		glog.Errorf("Failed to submit: %+v", err)
		w.WriteHeader(400)
		return
	}

	w.Header().Set("Content-Type", "text/plain")
	fmt.Fprintf(w, "%d", poseID)
}

func Submit(ctx context.Context, problemID int64, solution string) (int64, error) {
	poseID, err := submitToOfficial(problemID, solution)
	if err != nil {
		return 0, err
	}

	result, err := db.Execute(ctx,
		"INSERT INTO submissions" +
		"(problem_id, submission_data, submission_uuid, submission_submitted) " +
		"VALUES(?, ?, ?, CURRENT_TIMESTAMP())",
		problemID, solution, poseID)
	if err != nil {
		return 0, errors.Wrapf(err, "failed to insert a submission")
	}

	id, err := result.LastInsertId()
	if err != nil {
		return 0, errors.Wrapf(err, "failed to get an insert ID")
	}
	return id, nil
}

func submitToOfficial(problemID int64, solution string) (string, error) {
	glog.Infof("Problem ID: %d, solution: %s", problemID, solution)
	var vertices struct {
		Vertices [][]int64 `json:"vertices"`
	}
	if err := json.Unmarshal([]byte(solution), &vertices); err != nil {
		return "", errors.Wrapf(err, "failed to parse the solution: %+v", err)
	}
	if len(vertices.Vertices) == 0 {
		return "", errors.Errorf("No verticies are provided")
	}

	req, err := http.NewRequest(
		"POST",
		fmt.Sprintf("https://poses.live/api/problems/%d/solutions", problemID),
		bytes.NewBuffer([]byte(solution)))
	if err != nil {
		return "", errors.Wrapf(err, "failed to create a submission request")
	}
	req.Header.Set("Content-Type", "application/json")
	req.Header.Set("Authorization", "Bearer " + os.Getenv("UNAGI_API_KEY"))

	client := &http.Client{}
	resp, err := client.Do(req)
	if err != nil {
		return "", errors.Wrapf(err, "failed to submit a solution")
	}
	defer resp.Body.Close()

	buf, err := ioutil.ReadAll(resp.Body)
	if err != nil {
		return "", errors.Wrapf(err, "failed to read the submission response")
	}

	glog.Infof("Response: %s", string(buf))
	var response struct {
		ID string `json:"id"`
	}
	if err := json.Unmarshal(buf, &response); err != nil {
		return "", errors.Wrapf(err, "failed to parse a response: %+v", err)
	}
	return response.ID, nil
}
