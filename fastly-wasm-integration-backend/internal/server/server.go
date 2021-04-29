package server

import (
	"github.com/sirupsen/logrus"
	"net/http"
)

func Run() {
	fs := http.FileServer(http.Dir("./static"))

	http.Handle("/", middlewareController(fs.ServeHTTP))
	http.Handle("/login", middlewareController(loginHandler))

	logrus.Info("Listening on :5000...")
	err := http.ListenAndServe(":5000", nil)

	if err != nil {
		logrus.Fatal(err)
	}
}

func middlewareController(handler func(w http.ResponseWriter, r *http.Request)) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		setCorsAllowAll(w)
		handler(w, r)
	})
}

func setCorsAllowAll(w http.ResponseWriter) {
	w.Header().Set("Access-Control-Allow-Origin", "*")
	w.Header().Set("Access-Control-Allow-Credentials", "true")
	w.Header().Set("Access-Control-Allow-Methods", "POST, GET, OPTIONS")
	w.Header().Set("Access-Control-Allow-Headers", "Accept, Content-Type, Content-Length")
}
