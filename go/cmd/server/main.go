package main

import (
	"context"
	"flag"
	"github.com/golang/glog"
	"github.com/imos/icfpc2021/pkg/db"
	"net/http"

	_ "github.com/imos/icfpc2021/internal/handler"
)

var port = flag.String("port", ":8080", "API endpoint")

func handler(w http.ResponseWriter, r *http.Request) {
	glog.Info("Processing request...")
	var output int
	db.Cell(context.Background(), &output, "SELECT 1 + 1")
	glog.Infof("Output: %d", output)
	if r.Body == nil {
		glog.Errorf("body is empty")
		w.WriteHeader(400)
		return
	}
	defer r.Body.Close()
}

func main() {
	flag.Parse()
	glog.Info("Initializing...")
	//http.HandleFunc("/", handler)
	glog.Infof("Starting server on %s...", *port)
	if err := http.ListenAndServe(*port, nil); err != nil {
		glog.Fatal(err.Error())
	}
}
