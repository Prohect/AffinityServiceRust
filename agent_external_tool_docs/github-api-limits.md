# GitHub API Usage and Limits

This document summarizes the rules and observations regarding GitHub's API limits, specifically addressing issues like "Packet Too Large" or "Too Many Contents" errors encountered when using integrated tools like Zed or GitHub Copilot.

## 1. Official API Terms (Section H)

According to the [GitHub Terms of Service](https://docs.github.com/en/site-policy/github-terms/github-terms-of-service#h-api-terms):

*   **Abuse & Excessive Usage:** GitHub determines "abuse" or "excessive usage" at its sole discretion.
*   **Suspension:** Exceeding limits may result in temporary or permanent suspension of API access.
*   **Circumvention:** Sharing API tokens to exceed rate limitations is strictly prohibited.
*   **High-Throughput:** Users requiring higher limits should look into subscription-based API access.

## 2. Technical Limitations

### Payload Size (The "Packet" Issue)
Integrated AI tools (like Zed or Copilot) sometimes fail because they attempt to send too much context in a single request. 
*   **Symptoms:** "413 Payload Too Large" or connection resets.
*   **Root Cause:** The API gateway or the model's context window limits the amount of data sent in a single "packet" or request.

### Rate Limits
GitHub's REST and GraphQL APIs have specific rate limits:
*   **Authenticated Requests:** Usually 5,000 requests per hour.
*   **Search API:** Significantly lower (30 requests per minute).

## 3. Recommended Workarounds

When the standard `fetch` or integrated tools fail due to size limits:

1.  **Use `curl` for Large Files:** Download the raw content directly to bypass intermediate buffer limits.
    ```sh
    curl -L "URL" -o local_file.ext
    ```
2.  **Request Markdown Format:** GitHub Docs provides a Markdown-only endpoint that is much smaller than the full HTML page:
    ```sh
    curl -sL "https://docs.github.com/api/article/body?pathname=/path/to/doc"
    ```
3.  **Use CLI Search Tools:** Tools like `ddgr` (DuckDuckGo CLI) provide search results in a lightweight text format, avoiding the overhead of heavy API calls or browser rendering. check `docs\ddgr.md`
