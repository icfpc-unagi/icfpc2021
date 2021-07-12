package handler

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"io/ioutil"
	"math"
	"net/http"
	"regexp"
	"strconv"
	"strings"

	"github.com/golang/glog"
	"github.com/imos/icfpc2021/pkg/db"
	"github.com/pkg/errors"
)

func init() {
	http.HandleFunc("/problems", handleProblems)
}

func handleProblems(w http.ResponseWriter, r *http.Request) {
	ctx := r.Context()
	problems, err := getOfficialScores(ctx)
	if err != nil {
		fmt.Fprintf(w, "Failed to fetch official scores: %+v", err)
	}
	if err := renderProblems(ctx, w, problems); err != nil {
		glog.Errorf("Failed to render problems: %+v", err)
		return
	}
}

type Problem struct {
	ProblemID int64
	MyBestWithoutBonuses int64
	MyBest int64
	Current int64
	GlobalBest int64
	Scale int64
	TeamScoreWithoutBonuses int64
	TeamScore int64
}


type InputFigure struct {
	Edges    [][]int64
	Vertices [][]int64
}

type Input struct {
	Hole [][]int64
	Figure InputFigure `json:"figure"`
}

func renderProblems(ctx context.Context, w http.ResponseWriter, problems []Problem) error {
	var results []struct {
		ProblemID int64 `db:"problem_id"`
		SubmissionUseBonuses bool `db:"submission_use_bonuses"`
		SubmissionScore int64 `db:"submission_score"`
	}
	if err := db.Select(ctx, &results, `
SELECT
    problem_id,
    submission_use_bonuses,
    MIN(submission_score) AS submission_score
FROM
    (
    SELECT
        problem_id,
        COALESCE(
            submission_score,
            submission_estimated_score
        ) AS submission_score,
        submission_bonuses != "" AS submission_use_bonuses
    FROM
        submissions
) r
WHERE
    submission_score >= 0
GROUP BY
    problem_id,
    submission_use_bonuses
ORDER BY
    problem_id,
    submission_use_bonuses
`); err != nil {
		return errors.Wrapf(err, "failed to fetch best scores")
	}

	bestScores := map[int64]int64{}
	for _, r := range results {
		if !r.SubmissionUseBonuses {
			bestScores[r.ProblemID] = r.SubmissionScore
		}
	}
	for i := range problems {
		problem := &problems[i]
		if s, ok := bestScores[problem.ProblemID]; ok {
			problem.MyBestWithoutBonuses = s
		} else {
			problem.MyBestWithoutBonuses = -1
		}
	}

	bestScores = map[int64]int64{}
	for _, r := range results {
		if r.SubmissionUseBonuses {
			bestScores[r.ProblemID] = r.SubmissionScore
		}
	}
	for i := range problems {
		problem := &problems[i]
		if s, ok := bestScores[problem.ProblemID]; ok {
			problem.MyBest = s
		} else {
			problem.MyBest = -1
		}
	}

	for i := range problems {
		problem := &problems[i]
		data, _ := ioutil.ReadFile(
			fmt.Sprintf("/problems/%d.json", problem.ProblemID))
		var input Input
		json.Unmarshal(data, &input)
		problem.Scale = int64(
			len(input.Hole) * len(input.Figure.Vertices) * len(input.Figure.Edges))
		if problem.MyBestWithoutBonuses >= 0 {
			problem.TeamScoreWithoutBonuses = int64(math.Ceil(1000 * math.Log2(float64(problem.Scale) / 6) *
				math.Sqrt(float64(problem.GlobalBest+1)/float64(problem.MyBestWithoutBonuses+1))))
		}
		if problem.MyBest >= 0 {
			problem.TeamScore = int64(math.Ceil(1000 * math.Log2(float64(problem.Scale) / 6) *
				math.Sqrt(float64(problem.GlobalBest+1)/float64(problem.MyBest+1))))
		}
	}

	buf := &bytes.Buffer{}
	fmt.Fprintf(buf, "<h1>Problems</h1>\n")
	fmt.Fprintf(buf, `
<table class=table><tr><td>Problem ID</td>
<td>Score (my/global [remaining])</td>
<td colspan=1>Dislikes (best / with bonuses / current / global)</td>
</tr>
`)
	var score_sum int64
	for _, problem := range problems {
		global := int64(math.Ceil(1000 * math.Log2(float64(problem.Scale) / 6)))
		span := ""
		span_end := ""
		if problem.TeamScore < global {
			span = `<span style="color:red">`
			span_end = `</span>`
		}
		fmt.Fprintf(buf, `
<tr><td><a href="/static/show#problem_id=%d">%d</a></td>
<td align=right><code><a href="/static/show/#problem_id=%d&pose_url=%%2Fbest_solution%%3Fproblem_id%%3D%d">%5d</a> / %5d [%s%+6d%s] (%5d [%+6d])</code></td>
<td>(%d / %d / %d / %d)</td></tr>`,
			problem.ProblemID,
			problem.ProblemID,
			problem.ProblemID,
			problem.ProblemID,
			problem.TeamScoreWithoutBonuses,
			global,
			span,
			problem.TeamScoreWithoutBonuses - global,
			span_end,
			problem.TeamScore,
			problem.TeamScore - global,
			problem.MyBestWithoutBonuses,
			problem.MyBest,
			problem.Current,
			problem.GlobalBest)
			score_sum += problem.TeamScore
	}
	fmt.Fprintf(buf, "</table>")
	fmt.Fprintf(buf, "<code>Team Total Score: %8d</code>", score_sum)
	Template(w, buf.Bytes())
	return nil
}

func getOfficialScores(ctx context.Context) ([]Problem, error) {
	client, err := NewHTTPClient()
	req, err := http.NewRequest(
		"GET", "https://poses.live/problems", nil)

	if err != nil {
		return nil, errors.Wrapf(err, "failed to create a request")
	}

	resp, err := client.Do(req)
	if err != nil {
		return nil, errors.Wrapf(err, "failed to send a request")
	}

	buf, err := ioutil.ReadAll(resp.Body)
	if err != nil {
		return nil, errors.Wrapf(err, "failed to receive a response")
	}

	body := string(buf)
	if !strings.Contains(body, "<title>Problems</title>") {
		return nil, errors.Errorf("failed to parse problems page")
	}

	var problems []Problem
	rows := regexp.MustCompile(
		`<tr><td><a href="/problems/(\d+)">\d+</a></td><td>([^<]*)</td><td>([^<]*)</td></tr>`,
		).FindAllStringSubmatch(body, -1)
	for _, row := range rows {
		var problem Problem
		problem.ProblemID, _ = strconv.ParseInt(row[1], 10, 64)
		if myBest, err := strconv.ParseInt(row[2], 10, 64); err != nil {
			problem.Current = -1
		} else {
			problem.Current = myBest
		}
		if globalBest, err := strconv.ParseInt(row[3], 10, 64); err != nil {
			problem.GlobalBest = -1
		} else {
			problem.GlobalBest = globalBest
		}
		problems = append(problems, problem)
	}

	return problems, nil
}
