package middleware

import (
	"net/http"
	"regexp"

	"github.com/KJHJason/Blog/client/constants"
)

type Route struct {
	Method string
	Path   string
	Regex  *regexp.Regexp
}

func Auth(next http.Handler, whitelistedRoutes []Route) http.Handler {
	// validate routes
	for _, route := range whitelistedRoutes {
		if route.Path != "" && route.Regex != nil {
			panic("Route cannot have both Path and Regex")
		}
		if route.Path == "" && route.Regex == nil {
			panic("Route must have either Path or Regex")
		}
	}
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		_, err := r.Cookie(constants.SESSION_COOKIE_NAME)
		if err == nil {
			next.ServeHTTP(w, r)
			return
		}

		for _, route := range whitelistedRoutes {
			if r.Method == route.Method {
				if (route.Regex != nil && route.Regex.MatchString(r.URL.Path)) || r.URL.Path == route.Path {
					next.ServeHTTP(w, r)
					return
				}
			}
		}

		http.Redirect(w, r, "/admin", http.StatusTemporaryRedirect)
	})
}
