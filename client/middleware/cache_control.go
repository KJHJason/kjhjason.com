package middleware

import (
	"net/http"
	"regexp"
)

type CacheStrictPathValue struct {
	Path         string
	CacheControl string
}

type CachePathValue struct {
	Path         *regexp.Regexp
	CacheControl string
}

type CachePaths struct {
	StrictPaths []*CacheStrictPathValue
	RegexPaths  []*CachePathValue
}

func CacheControlMiddleware(next http.Handler, cachePaths *CachePaths) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if cachePaths == nil {
			next.ServeHTTP(w, r)
			return
		}
		for _, cache := range cachePaths.StrictPaths {
			if r.URL.Path == cache.Path {
				w.Header().Set("Cache-Control", cache.CacheControl)
				break
			}
		}
		for _, cache := range cachePaths.RegexPaths {
			if cache.Path.MatchString(r.URL.Path) {
				w.Header().Set("Cache-Control", cache.CacheControl)
				break
			}
		}
		next.ServeHTTP(w, r)
	})
}
