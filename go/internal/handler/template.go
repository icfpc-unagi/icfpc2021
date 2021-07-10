package handler

import (
	"fmt"
	"net/http"
)

func init() {
	http.HandleFunc("/template", handleTemplate)
}

func handleTemplate(w http.ResponseWriter, r *http.Request) {
	Template(w, []byte("<h1>test</h1>foobar"))
}

func Template(w http.ResponseWriter, body []byte) {
	fmt.Fprint(w,
`<html>
<header>
<meta charset="UTF-8">
<style>
html, body {
	position: relative;
	margin: 0;
	padding: 0;
	font-family: "Helvetica Neue", Helvetica, Arial, sans-serif;
	font-weight: 200;
}

table {
	font-family: "Helvetica Neue", Helvetica, Arial, sans-serif;
	font-weight: 300;
}

header {
	position: fixed;
	display: block;
	width: 100%;
	line-height: 50px;
	height: 50px;
	font-size: 18px;
	background: #eee;
	margin: 0px;
	font-weight: 200;
	vertical-align: middle;
}

header .title {
	display: inline-block;
	font-size: 150%;
	margin-right: 30px;
}

header .title a {
	text-decoration: none;
	color: inherit;
}

header .title a img {
	border: none;
}

.page {
	padding: 0 10%;
}

h1, h2, h3, h4, h5 {
	padding: 0;
	margin: 1ex 0 0.5ex;
}

h1 {
	font-size: 150%;
	font-weight: 200;
}

h2 {
	font-size: 140%;
	font-weight: 200;
}

h3 {
	font-size: 130%;
	font-weight: 200;
}

code {
	font-family: "Courier New", Consolas, monospace;
	font-weight: 200;
	white-space: pre-wrap;
}

.rank {
	font-family: "Courier New", Consolas, monospace;
	font-weight: 200;
}

.table {
	width: 100%;
	margin: 0.5ex;
	border: 1px solid #ccc;
	border-collapse: collapse;
	table-layout: fixed;
}

.table thead {
	text-align: center;
}

.table td {
	border: 1px solid #ccc;
	padding: 0.5ex;
	overflow-x: hidden;
	white-space: nowrap;
}

.table tr:nth-child(even), .table thead {
	background-color: #eee;
}

.ranking {
	border-collapse: collapse;
	table-layout: fixed;
	width: 100%;
}

.ranking tr:nth-child(even), .ranking thead {
	background-color: #eee;
}

.ranking td {
	padding: 0.3ex 1ex;
}

.ranking thead {
	font-weight: 400;
}

.monospace {
	font-family: "Courier New", Consolas, monospace;
	font-weight: 200;
}

form { margin: 0; }

.form {
	border: 1px solid #ccc;
	border-radius: 10px;
	padding: 10px;
	margin: 10px;
}

.form input {
	width: 100%;
	border: 1px solid #ccc;
	padding: 5px;
	font-size: 100%;
	font-weight: 300;
	margin: 5px;
}

.form input[type="submit"] {
	background-color: #0069d9;
	color: #ffffff;
	border-radius: 5px;
	width: inherit;
	padding: 5px 30px;
}

.form input:disabled[type="submit"] {
	background-color: #ffffff;
	color: #888888;
	border-radius: 5px;
	width: inherit;
	padding: 5px 30px;
}

.error, .success {
	margin: 10px 0;
	padding: 5px 20px;
	border: 1px solid #888;
	border-radius: 5px;
	font-weight: normal;
}

.error {
	border-color: #f00;
	background-color: #fee;
}

.success {
	border-color: #080;
	background-color: #efe;
}

</style>
</header>
<body>
<header>
<div class="page">
<div class="title">
<a href="/">Unagi ICFPC2021 Standings</a>
</div>

<a href="/"><s>Standings</s>🚧</a>
- <a href="/problems">Problems</a>
- <a href="/programs"><s>Programs</s>🚧</a>

</div>
</header>
<div class="page" style="padding-top: 60px">
`, string(body), ` 
</div>
</body>
</html>
`)
}
