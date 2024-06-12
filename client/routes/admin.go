package routes

import (
	"net/http"
)

func addAdmin(mux *http.ServeMux) {
	mux.HandleFunc("/admin/new/blog", func(w http.ResponseWriter, r *http.Request) {
		renderTemplate(w, r, "admin/new_blog.go.tmpl", map[string]any{})
	})
}
