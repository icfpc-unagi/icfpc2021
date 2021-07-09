package handler

import (
	"fmt"
	"github.com/golang/glog"
	"github.com/imos/icfpc2021/pkg/db"
	"github.com/pkg/errors"
	"io/ioutil"
	"net/http"
	"net/http/cookiejar"
	"net/url"
	"os"
	"regexp"
	"strconv"
	"strings"
)

func init() {
	http.HandleFunc("/update_score", handleUpdateScore)
}

func NewHTTPClient() (*http.Client, error) {
	jar, err := cookiejar.New(&cookiejar.Options{})
	if err != nil {
		return nil, errors.Wrapf(err, "failed to create a Cookie jar")
	}
	client := &http.Client{Jar: jar}
	values := url.Values{}
	values.Set("login.email", "icfpc-unagi"+"@"+"googlegroups.com")
	values.Add("login.password", os.Getenv("UNAGI_PORTAL_PASSWORD"))
	req, err := http.NewRequest("POST",
		"https://poses.live/login",
		strings.NewReader(values.Encode()))
	req.Header.Set("Content-Type", "application/x-www-form-urlencoded")
	if err != nil {
		return nil, errors.Wrapf(err, "failed to create a new request")
	}
	resp, err := client.Do(req)
	if err != nil {
		return nil, errors.Wrapf(err, "failed to sign in")
	}
	defer resp.Body.Close()
	_, err = ioutil.ReadAll(resp.Body)
	if err != nil {
		return nil, errors.Wrapf(err, "failed to receive the response")
	}
	return client, nil
}

func GetScore(client *http.Client, poseID string) (int64, error) {
	req, err := http.NewRequest(
		"GET", fmt.Sprintf("https://poses.live/solutions/%s", poseID), nil)
	if err != nil {
		return 0, errors.Wrapf(err, "failed to create a request")
	}

	resp, err := client.Do(req)
	if err != nil {
		return 0, errors.Wrapf(err, "failed to send a request")
	}

	buf, err := ioutil.ReadAll(resp.Body)
	if err != nil {
		return 0, errors.Wrapf(err, "failed to receive a response")
	}

	body := string(buf)
	if !strings.Contains(body, "<title>Pose</title>") {
		return 0, errors.Errorf("failed to parse score page")
	}

	matches := regexp.MustCompile(
		`<p>Dislikes: (\d+)</p>`).FindStringSubmatch(body)
	if len(matches) == 0 {
		return -1, nil
	}
	score, err := strconv.ParseInt(matches[1], 10, 64)
	if err != nil {
		return 0, errors.Errorf("failed to parse score: %+v", err)
	}
	return score, nil
}

func handleUpdateScore(w http.ResponseWriter, r *http.Request) {
	ctx := r.Context()
	var uuids []struct {
		SubmissionID string `db:"submission_id"`
		SubmissionUUID string `db:"submission_uuid"`
	}
	if err := db.Select(ctx, &uuids,
		`SELECT submission_id, submission_uuid FROM submissions `+
			`WHERE submission_uuid != "" AND submission_score IS NULL LIMIT 50`,
		); err != nil {
		glog.Errorf("Failed to fetch submission UUIDs: %+v", err)
		w.WriteHeader(500)
		return
	}

	client, err := NewHTTPClient()
	if err != nil {
		glog.Errorf("Failed to create an HTTP client: %+v", err)
		w.WriteHeader(500)
		return
	}

	score, err := GetScore(
		client, "706448af-19bc-4123-969e-c94895f65c2e")
	if score != 17801 {
		glog.Errorf("Something wrong with GetScore: %+v", err)
		return
	}

	for _, uuid := range uuids {
		score, err := GetScore(
			client, uuid.SubmissionUUID)
		if err != nil {
			glog.Errorf("Failed to get score: %+v", err)
			w.WriteHeader(500)
			return
		}
		glog.Infof("Update score: score=%d, uuid=%s",
			score, uuid.SubmissionUUID)
		db.Execute(ctx,
			"UPDATE submissions SET submission_score = ? "+
			"WHERE submission_id = ?",
			score, uuid.SubmissionID)
	}
}
