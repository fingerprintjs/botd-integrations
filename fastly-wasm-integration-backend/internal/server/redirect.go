package server

import (
	"bytes"
	"fmt"
	"io/ioutil"
	"net/http"
	"net/url"
)

func oldHandler(w http.ResponseWriter, r *http.Request) {
	newURL := "/redirect/new"
	var bdy = []byte(`login=humanLogin`)

	r.Method = "POST"
	r.URL, _ = url.Parse(newURL)
	r.RequestURI = newURL
	r.Body = ioutil.NopCloser(bytes.NewReader(bdy))
	r.Header.Set("Content-Type", "text/html")
	http.Redirect(w, r, newURL, 303)
}

func newHandler(w http.ResponseWriter, r *http.Request) {
	r.ParseForm()
	fmt.Printf("Method:%v\n", r.Method)
	fmt.Printf("Login:%v\n", r.Form.Get("login"))
}
