%.json:
	curl -H "Authorization: Bearer $$UNAGI_API_KEY" https://poses.live/api/problems/$* > $*.json.tmp
	mv $*.json.tmp $*.json
