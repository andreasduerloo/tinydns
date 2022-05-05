# tinydns

tinydns is a minimal DNS server.

V0.1 [work in progress]
- Reads a configuration file (see example)
- Listens for DNS requests on port 53 (UDP)
- Replies with the answer if it knows it
- Forwards the request if it doesn't, caches the response and answers
