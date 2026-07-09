const path = require('path');
const { LanguageClient, TransportKind } = require('vscode-languageclient/node');

function activate(context) {
  // Configurer le serveur LSP
  const rustBinExt = process.platform === 'win32' ? '.exe' : '';
  const lspPath = path.join(__dirname, '..', 'bin', 'rhl-lsp' + rustBinExt);

  const serverOptions = {
    run: {
      command: lspPath,
      transport: TransportKind.stdio,
      options: {
        env: { ...process.env },
      },
    },
    debug: {
      command: lspPath,
      transport: TransportKind.stdio,
      options: {
        env: { ...process.env, RUST_LOG: 'debug' },
      },
    },
  };

  const clientOptions = {
    documentSelector: [{ scheme: 'file', language: 'rhl' }],
  };

  const client = new LanguageClient('rhlLang', 'RHL Language Server', serverOptions, clientOptions);

  context.subscriptions.push(client.start());
}

module.exports = { activate };
