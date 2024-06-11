package middleware

import (
	"errors"
	"net/http"
	"strings"

	"github.com/KJHJason/Blog/client/constants"
)

func Csrf(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		_, err := r.Cookie("csrf-token")
		if errors.Is(err, http.ErrNoCookie) {
			// get requestedUrl for the API to redirect the user back to the requested page
			requestedUrl := strings.TrimPrefix(r.URL.Path, "/")
			if r.URL.RawQuery != "" {
				requestedUrl += "?" + r.URL.RawQuery
			}
			if r.URL.Fragment != "" {
				requestedUrl += "#" + r.URL.Fragment
			}

			toRedirect := constants.GetApiUrl() + "/csrf-token?redirect=" + requestedUrl
			http.Redirect(w, r, toRedirect, http.StatusSeeOther)
			return
		}

		next.ServeHTTP(w, r)
	})
}
