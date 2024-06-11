package routes

import (
	"fmt"
	"html/template"
	"net/http"

	"github.com/KJHJason/Blog/client/constants"
	"github.com/KJHJason/Blog/client/middleware"
)

func renderTemplate(w http.ResponseWriter, r *http.Request, tmplPath string, data map[string]any) {
	tmplPath = fmt.Sprintf("templates/%s", tmplPath)

	t := template.New("base.go.tmpl").Funcs(template.FuncMap{
		"nonce": func() string {
			nonce := middleware.GetNonce(r)
			return nonce
		},
		"apiUrl": func() string {
			return constants.GetApiUrl()
		},
		"csrfToken": func() string {
			csrfToken := middleware.GetCsrfToken(r)
			return csrfToken
		},
		"csrfHeader": func() string {
			return constants.CSRF_HEADER_NAME
		},
	})
	t = template.Must(t.ParseFiles(
		"templates/base.go.tmpl",
		tmplPath,
	))
	t.Execute(w, data)
}

func AddRoutes(mux *http.ServeMux) {
	addGeneral(mux)
	addAuth(mux)
}
