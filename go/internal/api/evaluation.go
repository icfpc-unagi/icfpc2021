package api

type Evaluation struct {
	Bonuses interface{} `json:"bonuses"`
	ObtainedBonuses interface{} `json:"obtained_bonuses"`
	Dislikes int64 `json:"dislikes"`
}
