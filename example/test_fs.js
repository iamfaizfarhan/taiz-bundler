// example/test_fs.js
(async () => {
    try {
      const testData = { message: "Hello, Taiz!" };
      await taiz.fs.write('test.json', JSON.stringify(testData));
      const saved = await taiz.fs.read('test.json');
      console.log('FS Read Result:', saved);
      if (saved.message === testData.message) {
        console.log('FS Test: PASS');
      } else {
        console.log('FS Test: FAIL');
      }
    } catch (err) {
      console.error('FS Error:', err);
    }
  })();