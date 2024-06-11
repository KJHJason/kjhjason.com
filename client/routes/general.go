package routes

import (
	"net/http"
)

func addGeneral(mux *http.ServeMux) {
	mux.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		renderTemplate(w, r, "general/index.go.tmpl", nil)
	})
	mux.HandleFunc("/experiences", func(w http.ResponseWriter, r *http.Request) {
		renderTemplate(w, r, "general/experiences.go.tmpl", nil)
	})
	mux.HandleFunc("/projects", func(w http.ResponseWriter, r *http.Request) {
		renderTemplate(w, r, "general/projects.go.tmpl", nil)
	})
	mux.HandleFunc("/skills", func(w http.ResponseWriter, r *http.Request) {
		renderTemplate(w, r, "general/skills.go.tmpl", nil)
	})
	mux.HandleFunc("/blog", func(w http.ResponseWriter, r *http.Request) {
		renderTemplate(w, r, "general/blog.go.tmpl", nil)
	})
}
