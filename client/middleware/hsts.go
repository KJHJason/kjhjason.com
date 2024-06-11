package middleware

import (
	"fmt"
	"net/http"
)

type HstsOptions struct {
	MaxAge           int
	IncludeSubDomain bool
	Preload          bool // IncludeSubDomain must be true for Preload to work
}

func Hsts(next http.Handler, opt HstsOptions) http.Handler {
	value := fmt.Sprintf("max-age=%d", opt.MaxAge)
	if opt.IncludeSubDomain {
		value += "; includeSubDomains"
		if opt.Preload {
			value += "; preload"
		}
	}
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Strict-Transport-Security", value)
		next.ServeHTTP(w, r)
	})
}
