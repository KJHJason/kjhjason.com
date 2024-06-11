package middleware

import (
	"log"
	"net/http"
)

func Logger(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		userAgent := r.Header.Get("User-Agent")
		if userAgent == "" {
			userAgent = "Unknown"
		}

		log.Printf("%s %s [%s]", r.Method, r.URL.Path, userAgent)
		next.ServeHTTP(w, r)
	})
}
