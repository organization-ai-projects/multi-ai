const fs = require('fs');
const path = require('path');

const rustBinExt = process.platform === 'win32' ? '.exe' : '';
const binDir = path.join(__dirname, '..', 'bin');
const srcBin = path.join(__dirname, '..', '..', 'target', 'debug', 'rhl-lsp' + rustBinExt);
const dstBin = path.join(binDir, 'rhl-lsp' + rustBinExt);

if (!fs.existsSync(binDir)) {
  fs.mkdirSync(binDir, { recursive: true });
}

fs.copyFileSync(srcBin, dstBin);
