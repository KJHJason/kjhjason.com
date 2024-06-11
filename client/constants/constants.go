package constants

const (
	DEBUG_MODE = true

	// Note: Remember to change in the Rust API as well
	CSRF_COOKIE_NAME = "csrf-token"
	CSRF_HEADER_NAME = "X-CSRF-Token"

	API_URL = "https://api.kjhjason.com"
)

func GetApiUrl() string {
	if DEBUG_MODE {
		return "http://localhost:8080"
	}
	return API_URL
}
