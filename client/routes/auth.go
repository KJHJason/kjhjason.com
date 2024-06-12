package routes

import (
	"net/http"
)

func addAuth(mux *http.ServeMux) {
	mux.HandleFunc("/login", func(w http.ResponseWriter, r *http.Request) {
		http.Redirect(w, r, "/admin", http.StatusTemporaryRedirect)
	})
	mux.HandleFunc("/admin", func(w http.ResponseWriter, r *http.Request) {
		renderTemplate(w, r, "auth/login.go.tmpl", map[string]any{
			"loginUrl": "admin",
		})
	})
	mux.HandleFunc("/auth/login", func(w http.ResponseWriter, r *http.Request) {
		renderTemplate(w, r, "auth/login.go.tmpl", map[string]any{
			"loginUrl": "auth/login",
		})
	})
}
