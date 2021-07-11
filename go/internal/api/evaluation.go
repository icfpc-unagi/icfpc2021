package api

type Evaluation struct {
	Dislikes int64 `json:"dislikes"`
	Bonuses interface{} `json:"bonuses"`
	ObtainedBonuses interface{} `json:"obtained_bonuses"`

	BonusesStr string `json:"-"`
	ObtainedBonusesStr string `json:"-"`
	BonusesHash string `json:"-"`
}
