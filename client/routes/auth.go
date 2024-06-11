package routes

import (
	"net/http"
)

func addAuth(mux *http.ServeMux) {
	mux.HandleFunc("/login", func(w http.ResponseWriter, r *http.Request) {
		renderTemplate(w, r, "auth/login.go.tmpl", nil)
	})
}
