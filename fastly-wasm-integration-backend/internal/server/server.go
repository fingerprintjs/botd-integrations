package server

import (
	"github.com/sirupsen/logrus"
	"net/http"
)

func Run() {
	fs := http.FileServer(http.Dir("./static/main"))
	captchaFS := http.FileServer(http.Dir("./static/captcha"))
	redirectFS := http.FileServer(http.Dir("./static/redirect"))

	http.Handle("/", middlewareController(fs))
	http.HandleFunc("/login", loginHandler)
	http.Handle("/captcha", middlewareController(captchaFS))
	http.Handle("/redirect", middlewareController(redirectFS))

	http.HandleFunc("/redirect/old", oldHandler)
	http.HandleFunc("/redirect/new", newHandler)

	logrus.Info("Listening on :5001...")
	err := http.ListenAndServe(":5001", nil)

	if err != nil {
		logrus.Fatal(err)
	}
}

func middlewareController(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		logrus.Info("Hello from Middleware")
		setCorsAllowAll(w)
		next.ServeHTTP(w, r)
	})
}

func setCorsAllowAll(w http.ResponseWriter) {
	w.Header().Set("Access-Control-Allow-Origin", "*")
	w.Header().Set("Access-Control-Allow-Credentials", "true")
	w.Header().Set("Access-Control-Allow-Methods", "POST, GET, OPTIONS")
	w.Header().Set("Access-Control-Allow-Headers", "Accept, Content-Type, Content-Length")
}
