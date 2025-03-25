// example/test_worker.js
(async () => {
    try {
      const input = [1, 2, 3, 4, 5];
      const buffer = new TextEncoder().encode(JSON.stringify(input)).buffer;
      const result = await taiz.worker((buf) => {
        const decoder = new TextDecoder();
        const data = JSON.parse(decoder.decode(new Uint8Array(buf)));
        return data.map(x => x * 2);
      }, buffer);
      console.log('Worker Result:', result);
      if (result.join(',') === '2,4,6,8,10') {
        console.log('Worker Test: PASS');
      } else {
        console.log('Worker Test: FAIL');
      }
    } catch (err) {
      console.error('Worker Error:', err);
    }
  })();