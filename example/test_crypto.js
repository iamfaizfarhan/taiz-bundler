// example/test_crypto.js
(async () => {
    try {
      const input = "Hello, Taiz!";
      const expectedHash = "dffd6021bb2bd5b0af676290809ec3a53191dd81c7f70a4b28688a362182986f"; // SHA-256 of "Hello, Taiz!"
      const hash = await taiz.crypto.sha256(input);
      console.log('Crypto Result:', hash);
      if (hash === expectedHash) {
        console.log('Crypto Test: PASS');
      } else {
        console.log('Crypto Test: FAIL');
      }
    } catch (err) {
      console.error('Crypto Error:', err);
    }
  })();