package handler

import (
	"bytes"
	"context"
	"fmt"
	"github.com/golang/glog"
	"github.com/imos/icfpc2021/pkg/db"
	"github.com/pkg/errors"
	"net/http"
	"sort"
)

func init() {
	http.HandleFunc("/bonuses", handleBonuses)
}

func handleBonuses(w http.ResponseWriter, r *http.Request) {
	ctx := r.Context()
	bonuses, err := getSubmissionBonuses(ctx)
	if err != nil {
		fmt.Fprintf(w, "Failed to fetch bonuses: %+v", err)
	}
	if err := renderSubmissionBonuses(ctx, w, bonuses); err != nil {
		glog.Errorf("Failed to render problems: %+v", err)
		return
	}
}

func renderSubmissionBonuses(ctx context.Context, w http.ResponseWriter, bonuses []SubmissionBonus) error {
	bonusesMap := map[int64][]SubmissionBonus{}
	for _, bonus := range bonuses {
		bonusesMap[bonus.ProblemID] = append(bonusesMap[bonus.ProblemID], bonus)
	}
	problemBonuses := [][]SubmissionBonus{}
	for _, bonuses := range bonusesMap {
		problemBonuses = append(problemBonuses, bonuses)
	}
	sort.Slice(problemBonuses, func(i, j int) bool {
		return problemBonuses[i][0].ProblemID < problemBonuses[j][0].ProblemID
	})

	buf := &bytes.Buffer{}
	fmt.Fprintf(buf, "<h1>Bonuses</h1>\n")
	fmt.Fprint(buf, `<table class=table><tr><td width="10%">Problem ID</td><td>Required</td><td>Obtained</td><td width="10%">Score</td></tr>`)
	for _, bonuses := range problemBonuses {
		for i, bonus := range bonuses {
			fmt.Fprintf(buf, "<tr>")
			if i == 0 {
				fmt.Fprintf(buf, "<td rowspan=%d>%d</td>", len(bonuses), bonus.ProblemID)
			}
			fmt.Fprintf(buf, "<td>%s</td>", bonus.SubmissionBonuses)
			fmt.Fprintf(buf, "<td>%s</td>", bonus.SubmissionObtainedBonuses)
			fmt.Fprintf(buf, "<td>%d</td>", bonus.SubmissionEstimatedScore)
			fmt.Fprintf(buf, "</tr>")
		}
	}
	fmt.Fprintf(buf, "</table>")
	Template(w, buf.Bytes())
	return nil
}

type SubmissionBonus struct {
	SubmissionID int64 `db:"submission_id"`
	ProblemID int64 `db:"problem_id"`
	SubmissionBonuses string `db:"submission_bonuses"`
	SubmissionObtainedBonuses string `db:"submission_obtained_bonuses"`
	SubmissionBonusesHash string `db:"submission_bonuses_hash"`
	SubmissionEstimatedScore int64 `db:"submission_estimated_score"`
}

func getSubmissionBonuses(ctx context.Context) ([]SubmissionBonus, error) {
	var results []SubmissionBonus
	err := db.Select(ctx, &results, `
SELECT
    MIN(submission_id) AS submission_id,
    problem_id,
    MIN(submission_bonuses) submission_bonuses,
    MIN(submission_obtained_bonuses) submission_obtained_bonuses,
    submission_bonuses_hash,
    MIN(submission_estimated_score) submission_estimated_score
FROM
    submissions
NATURAL JOIN(
    SELECT
        MIN(submission_id) AS submission_id
    FROM
        submissions
    NATURAL JOIN(
        SELECT
            problem_id,
            submission_bonuses_hash,
            MIN(submission_estimated_score) AS submission_estimated_score
        FROM
            submissions
        WHERE
            submission_estimated_score >= 0
        GROUP BY
            problem_id,
            submission_bonuses_hash
    ) r
    GROUP BY
        problem_id,
        submission_bonuses_hash
) r
GROUP BY
    problem_id,
    submission_bonuses_hash
ORDER BY
    problem_id,
    submission_bonuses,
    submission_obtained_bonuses
`)
	if err != nil {
		return nil, errors.Wrapf(err, "failed to list submission bonuses")
	}
	return results, nil
}
