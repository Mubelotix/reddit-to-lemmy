// ClientRequest HTTP/1.1 POST https://www.reddit.com/auth/v2/oauth/access-token/loid
//   headers:
//     "content-type": "application/json; charset=UTF-8"
//     "content-length": "30"
//     "x-reddit-compression": "1"
//     "x-attestation-device-token": "eyJhbGciOiJFZERTQSIsImtpZCI6IlNIQTI1NjpiUDlrUWNMZFFaV2RFMzFUWXRjclBhT0N0UHc2S3ZsVmNNWjNFVTI0WVQ4IiwidHlwIjoiSldUIn0.eyJleHAiOjE3MzkyMDQ5ODIsImlhdCI6MTczNzk5NTM4MiwianRpIjoiRXZmSTBEV1lXTWY5bWI1YUtnSDlDTFl3QjdNIiwiY2lkIjoib2hYcG9xclpZdWIxa2ciLCJhdHQiOjIsImFlYyI6MjB9.AQM0eZl-szpBrZK8ZIBqTiRu4N9GhKbGhtDeeZ6AavyuHtfmxoqZWLmdDXbfVQC_dskRHT49z0Xv_1sLwjSlDg"
//     "cookie": "code-server-session=%24argon2id%24v%3D19%24m%3D65536%2Ct%3D3%2Cp%3D4%24TwFidMxTlb2MNfPgWr84RQ%24SnMV9sx20OG0eecGXsItNZw%2FM%2Fb8vVKLWqAgBV07zSY"
//     "x-reddit-qos": "down-rate-mbps=127.269"
//     "x-reddit-loid": "00000000005xrogxaw.2.1584284301614.Z0FBQUFBQm5tUFl6bTJsZXZKRi16RXBPLUhmeTZISm9oNjg3NGVYb05DczZsNS1YdnlFaUVaSFNsb29BZEF1R3NoSUQzSHpCZWhOaGRDcHJOM1lwYW1LODFmNzRna01HRHk2a1ZXQnVOTkdpWW1lQlQ5V1ZuNU1KYkNBVzBXdzEwVUg0UWR4cVRyWUI"
//     "authorization": "Basic b2hYcG9xclpZdWIxa2c6"
//     "client-vendor-id": "6918b40d-1a50-4478-8b05-ad96845c1ae3"
//     "user-agent": "Reddit/Version 2025.03.1/Build 2181077/Android 15"
//     "x-reddit-media-codecs": "available-codecs=video/avc, video/hevc, video/x-vnd.on2.vp9"
//     "accept-encoding": "gzip"

// {"scopes":["*","email","pii"]}
// HttpResponse { error: None, res: 
// Response HTTP/1.1 200 OK
//   headers:
//     "nel": "{\"report_to\": \"w3-reporting-nel\", \"max_age\": 14400, \"include_subdomains\": false, \"success_fraction\": 1.0, \"failure_fraction\": 1.0}"
//     "content-type": "application/json"
//     "vary": "Accept-Encoding"
//     "date": "Tue, 28 Jan 2025 16:48:19 GMT"
//     "x-xss-protection": "1; mode=block"
//     "set-cookie": "edgebucket=sYuIWoNYTbRyf1XM4l; Domain=reddit.com; Max-Age=63071999; Path=/;  secure"
//     "x-content-type-options": "nosniff"
//     "cache-control": "private, max-age=3600"
//     "report-to": "{\"group\": \"w3-reporting-nel\", \"max_age\": 14400, \"include_subdomains\": true,  \"endpoints\": [{ \"url\": \"https://w3-reporting-nel.reddit.com/reports\" }]}, {\"group\": \"w3-reporting\", \"max_age\": 14400, \"include_subdomains\": true, \"endpoints\": [{ \"url\": \"https://w3-reporting.reddit.com/reports\" }]}, {\"group\": \"w3-reporting-csp\", \"max_age\": 14400, \"include_subdomains\": true, \"endpoints\": [{ \"url\": \"https://w3-reporting-csp.reddit.com/reports\" }]}"
//     "strict-transport-security": "max-age=31536000; includeSubdomains"
//     "via": "1.1 varnish"
//     "content-length": "1355"
//     "x-reddit-session": "oceeqlogrkaebqqldg.0.1738082899765.Z0FBQUFBQm5tUXBUenotbHVEVlJhX3BXVHJUdkxjcmR1ZVg2Qk13X1NiMlljbnBwc2tVRGdmeVRtdVRkWE9VVHdjQ0dZT1hBUkFhMDFYc1JtaDRqdlgxSGpkWkhhZDF3SmFvMDJWOEZoVWp3cEVzTTBMNmZtdExLQ1BpdTJia2xQb1FrNWJfbE1VRnA"
//     "accept-ranges": "bytes"
//     "server": "snooserv"
//     "x-frame-options": "SAMEORIGIN"
//   body: Sized(1355)
//  }
// {"access_token":"eyJhbGciOiJSUzI1NiIsImtpZCI6IlNIQTI1NjpzS3dsMnlsV0VtMjVmcXhwTU40cWY4MXE2OWFFdWFyMnpLMUdhVGxjdWNZIiwidHlwIjoiSldUIn0.eyJzdWIiOiJsb2lkIiwiZXhwIjoxNzM4MTY5Mjk5Ljc2ODgwNywiaWF0IjoxNzM4MDgyODk5Ljc2ODgwNywianRpIjoiNGpxc1Q1VWRGZV9fZlgwZGpRQldfMXFhaFNLY3ZBIiwiY2lkIjoib2hYcG9xclpZdWIxa2ciLCJsaWQiOiJ0Ml81eHJvZ3hhdyIsImxjYSI6MTU4NDI4NDMwMTYxNCwic2NwIjoiZUp4a2tkR090REFJaGQtRmE1X2dmNVVfbTAxdGNZYXNMUWFvazNuN0RWb2NrNzN5d1BuYXd2RV9ZRTIwd2dRYkVVeVFjdWJlekZWcjNGdkdpczBVSnNpQ3BkQWhzWkFUeTVwSVlJSW5xYkc4WVFJcTJJek01VW83MXRUU0EyR0N5aVZ6Vy1oeGFST2F1N0hvMlltcktwZVZCelhtcWx3YTI3aUY3WWx4Wm1PMUlSWFg1VlFtYVZrb244V0xmbnluLXRZLWY2YmZoUFprV0ZFMVBkQTdncWtjbjQzRkY5TzAtM3RxMHJOMXdhTDJYZzlTLTF5UDNiWFBtb1ZtNV9aelBIOXRKT055WEpxZnlmN2Q5VXZvd0NQUHk0dzZnQkZyLUZHR0hWbUhmOVVCVkM0b3lWaUMtRFFDdVhMNTlsLUM3WnIwcjNFNzRvdHZSdHowaHQtYWdYb3VRUno2Tkw1LUF3QUFfXy1xZWQ0XyIsImZsbyI6MSwiZGEiOnsic3QiOjIsImVjIjoyMCwiYWF0IjoxNzM3OTk1MzgyfX0.YWxYeLsv64lRjJllkwY-V-s2VSK-hhhvJpc0SCE4m1PzeGUDoMD4GjQS7JbRxm-RSLbPa5pOjB0thfJBIKrSAwJnYMqO5OBD-zA7gXSeInwma4_0lCx-YuKWNRBBM5cTg6vzV7FqrdsOz4HWjPEFNEpFmRA4qcWaoyLoxJ02ntONvRQDkBhfnL16jFPk9sCkVeevUt3fPDMXTBlkrw4m-WbkfS8T1JgYIZ7yylxeErud9LU5ELkYZB45KnElx9TXVL-xuKFvvO5g-6--j8rfBc3A3gxvC6XWPbRbQgyMx5BRzm52Kgb0VTmn-FK3YojyK39u0aw8frUJn19Qyig1tQ", "expiry_ts":1738169299, "expires_in":86399, "scope":["*", "email", "pii"], "token_type":"bearer"}
