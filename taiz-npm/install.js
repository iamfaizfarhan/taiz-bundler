const { execSync } = require('child_process');
const fs = require('fs');
const path = require('path');

const binDir = path.join(__dirname, 'bin');
const binaryPath = path.join(binDir, 'taiz-cli');

if (!fs.existsSync(binDir)) {
  fs.mkdirSync(binDir);
}

if (!fs.existsSync(binaryPath)) {
  try {
    console.log('Installing taiz-cli binary...');
    execSync('cargo install taiz-cli --root .', { stdio: 'inherit' });
    fs.renameSync(path.join(__dirname, 'bin/taiz-cli'), binaryPath);
  } catch (err) {
    console.error('Failed to install taiz-cli:', err);
    process.exit(1);
  }
}