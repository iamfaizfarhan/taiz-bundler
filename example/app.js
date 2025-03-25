(async () => {
    const data = await taiz.fetch('https://jsonplaceholder.typicode.com/posts');
    const hash = await taiz.crypto.sha256(JSON.stringify(data));
    console.log('Hash:', hash);
    const encoder = new TextEncoder();
    const buffer = encoder.encode(JSON.stringify(data)).buffer;
    const processed = await taiz.worker((buf) => {
      const decoder = new TextDecoder();
      const posts = JSON.parse(decoder.decode(new Uint8Array(buf)));
      return posts.map(p => ({ ...p, id: p.id * 2 }));
    }, buffer);
    await taiz.fs.write('processed.json', JSON.stringify(processed));
    const saved = await taiz.fs.read('processed.json');
    console.log('Processed:', saved);
  })();