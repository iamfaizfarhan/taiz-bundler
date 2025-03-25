(function () {
    const env = typeof window !== 'undefined' ? 0 : // Browser
                typeof Deno !== 'undefined' ? 2 :   // Deno
                1;                                  // Node
  
    // Fetch implementations
    const fetchImpls = [
      (url, opts) => window.fetch(url, opts).then(r => r.json()), // Browser
      (url, opts) => require('node-fetch')(url, opts).then(r => r.json()), // Node
      (url, opts) => Deno.fetch(url, opts).then(r => r.json()) // Deno
    ];
  
    // Unified Taiz API
    globalThis.taiz = {
      fetch: fetchImpls[env]
    };
  })();