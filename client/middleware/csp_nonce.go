package middleware

import (
	"context"
	"crypto/rand"
	"encoding/base64"
	"fmt"
	"log"
	"net/http"
	"strings"
)

const (
	exemptCSPHeader = "X-Exempt-CSP"
	exemptCSPValue  = "1"
)

type ctxKeyNonce struct{}

// ContentSecurityPolicies defines the allowed CSP policies.
type ContentSecurityPolicies struct {
	ScriptSrc      []string
	StyleSrc       []string
	DefaultSrc     []string
	BaseURI        []string
	ImgSrc         []string
	FontSrc        []string
	ObjectSrc      []string
	FormAction     []string
	FrameAncestors []string
	// Add other CSP directives as needed
}

var defaultCSP = ContentSecurityPolicies{
	ScriptSrc:      []string{"'self'"},
	StyleSrc:       []string{"'self'", "https:", "'unsafe-inline'"},
	DefaultSrc:     []string{"'self'"},
	BaseURI:        []string{"'self'"},
	ImgSrc:         []string{"'self'", "data:"},
	FontSrc:        []string{"'self'", "https:", "data:"},
	ObjectSrc:      []string{"'none'"},
	FrameAncestors: []string{"'self'"},
}

func generateNonce(nBytes int) (string, error) {
	nonce := make([]byte, nBytes)
	_, err := rand.Read(nonce)
	if err != nil {
		return "", err
	}
	return base64.StdEncoding.EncodeToString(nonce), nil
}

func ExemptRouteFromCsp(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set(exemptCSPHeader, exemptCSPValue)
		next.ServeHTTP(w, r)
	})
}

func CspNonce(next http.Handler, csp *ContentSecurityPolicies, nonceSize int) http.Handler {
	if csp == nil {
		csp = &defaultCSP
	}
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if strings.HasPrefix(r.URL.Path, "/static/") {
			next.ServeHTTP(w, r)
			return
		}

		nonce, err := generateNonce(nonceSize)
		if err != nil {
			http.Error(w, "Internal Server Error", http.StatusInternalServerError)
			return
		}

		ctx := context.WithValue(r.Context(), ctxKeyNonce{}, nonce)
		r = r.WithContext(ctx)

		// Check if the route is exempt from CSP
		if w.Header().Get(exemptCSPHeader) == exemptCSPValue {
			next.ServeHTTP(w, r)
			return
		}

		// Construct the CSP header
		csp := buildCspHeader(csp, nonce)
		w.Header().Set("Content-Security-Policy", csp)
		next.ServeHTTP(w, r)
	})
}

func addDirectives(directives []string, name string, values []string) []string {
	if len(values) > 0 {
		directives = append(directives, fmt.Sprintf("%s %s", name, strings.Join(values, " ")))
	}
	return directives
}

func buildCspHeader(csp *ContentSecurityPolicies, nonce string) string {
	if csp == nil {
		csp = &defaultCSP
	}

	directives := make([]string, 0)
	addDirectives(directives, "default-src", csp.DefaultSrc)
	addDirectives(directives, "script-src 'nonce-"+nonce+"'", csp.ScriptSrc)
	addDirectives(directives, "style-src 'nonce-"+nonce+"'", csp.StyleSrc)
	addDirectives(directives, "base-uri", csp.BaseURI)
	addDirectives(directives, "img-src", csp.ImgSrc)
	addDirectives(directives, "font-src", csp.FontSrc)
	addDirectives(directives, "object-src", csp.ObjectSrc)
	addDirectives(directives, "form-action", csp.FormAction)
	addDirectives(directives, "frame-ancestors", csp.FrameAncestors)
	return strings.Join(directives, "; ")
}

func GetNonce(r *http.Request) string {
	nonce, ok := r.Context().Value(ctxKeyNonce{}).(string)
	if !ok {
		log.Println("Failed to convert nonce to string")
		return ""
	}
	return nonce
}
