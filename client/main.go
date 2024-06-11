package main

import (
	"fmt"
	"html/template"
	"log"
	"net/http"
	"regexp"

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
	})
	t = template.Must(t.ParseFiles(
		"templates/base.go.tmpl",
		tmplPath,
	))
	t.Execute(w, data)
}

func main() {
	fmt.Println("Client running on http://localhost:8000")

	http.Handle(
		"/static/",
		http.StripPrefix("/static/", http.FileServer(http.Dir("static"))),
	)
	http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		renderTemplate(w, r, "general/index.go.tmpl", nil)
	})
	http.HandleFunc("/experiences", func(w http.ResponseWriter, r *http.Request) {
		renderTemplate(w, r, "general/experiences.go.tmpl", nil)
	})
	http.HandleFunc("/projects", func(w http.ResponseWriter, r *http.Request) {
		renderTemplate(w, r, "general/projects.go.tmpl", nil)
	})
	http.HandleFunc("/skills", func(w http.ResponseWriter, r *http.Request) {
		renderTemplate(w, r, "general/skills.go.tmpl", nil)
	})
	http.HandleFunc("/blog", func(w http.ResponseWriter, r *http.Request) {
		renderTemplate(w, r, "general/blog.go.tmpl", nil)
	})

	csp_options := middleware.ContentSecurityPolicies{
		ScriptSrc: []string{
			"'self'",
			"https://unpkg.com/htmx.org@1.9.12",
		},
		StyleSrc: []string{
			"'self'",
		},
	}
	mainHandler := middleware.CspNonceMiddleware(http.DefaultServeMux, &csp_options, 32)
	mainHandler = middleware.ContentTypeMiddleware(mainHandler)
	mainHandler = middleware.LoggerMiddleware(mainHandler)

	var cachePaths *middleware.CachePaths
	if !constants.DEBUG_MODE {
		mainHandler = middleware.HstsMiddleware(mainHandler, middleware.HstsOptions{
			MaxAge:           31536000, // 1 year
			IncludeSubDomain: true,
			Preload:          true,
		})
		cachePaths = &middleware.CachePaths{
			StrictPaths: []*middleware.CacheStrictPathValue{
				{Path: "/", CacheControl: "public, max-age=86400"},                                // 1 day
				{Path: "/favicon.ico", CacheControl: "public, max-age=31536000, must-revalidate"}, // 1 year
			},
			RegexPaths: []*middleware.CachePathValue{
				{Path: regexp.MustCompile(`^/static/.*$`), CacheControl: "public, max-age=31536000, must-revalidate"}, // 1 year
			},
		}
	}
	mainHandler = middleware.CacheControlMiddleware(mainHandler, cachePaths)
	log.Fatal(http.ListenAndServe(":8000", mainHandler))
}
