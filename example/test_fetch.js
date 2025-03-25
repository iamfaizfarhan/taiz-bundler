// example/test_fetch.js
(async () => {
    try {
      const data = await taiz.fetch('https://jsonplaceholder.typicode.com/posts/1');
      console.log('Fetch Result:', data);
      if (data.id === 1 && data.title) {
        console.log('Fetch Test: PASS');
      } else {
        console.log('Fetch Test: FAIL');
      }
    } catch (err) {
      console.error('Fetch Error:', err);
    }
  })();