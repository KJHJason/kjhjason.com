package main

import (
	"fmt"
	"io"
	"log"
	"net/http"
	"os"
	"regexp"

	"github.com/KJHJason/Blog/client/constants"
	"github.com/KJHJason/Blog/client/middleware"
	"github.com/KJHJason/Blog/client/routes"
)

func main() {
	fmt.Println("Client running on http://localhost:8000")

	mux := http.NewServeMux()
	routes.AddRoutes(mux)
	mux.Handle(
		"/static/",
		http.StripPrefix("/static/", http.FileServer(http.Dir("static"))),
	)
	mux.HandleFunc("/favicon.ico", func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Content-Type", "image/x-icon")
		favicon, err := os.OpenFile("./static/images/favicon.ico", os.O_RDONLY, 0)
		if err != nil {
			http.Error(w, "Favicon not found", http.StatusNotFound)
			return
		}
		defer favicon.Close()
		_, err = io.Copy(w, favicon)
		if err != nil {
			http.Error(w, "Error serving favicon", http.StatusInternalServerError)
		}
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
	mainHandler := middleware.CspNonce(mux, &csp_options, 32)
	mainHandler = middleware.ContentType(mainHandler)
	mainHandler = middleware.Logger(mainHandler)
	mainHandler = middleware.Csrf(mainHandler)

	var cachePaths *middleware.CachePaths
	if !constants.DEBUG_MODE {
		mainHandler = middleware.Hsts(mainHandler, middleware.HstsOptions{
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
	mainHandler = middleware.CacheControl(mainHandler, cachePaths)
	log.Fatal(http.ListenAndServe(":8000", mainHandler))
}
