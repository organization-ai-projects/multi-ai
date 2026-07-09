#!/bin/bash

# 1. Détection de VSCode (Windows)
VSCODE_PATH="/c/Users/$USERNAME/AppData/Local/Programs/Microsoft VS Code/bin/code.cmd"
if [ ! -f "$VSCODE_PATH" ]; then
    echo "❌ VSCode non trouvé à l’emplacement standard."
    exit 1
fi

# 2. Vérification de vsce
if ! command -v vsce &> /dev/null; then
    echo "❌ 'vsce' est requis. Installe avec : pnpm add -g @vscode/vsce"
    exit 1
fi

# 3. Installation et packaging
pnpm install

# (Optionnel) pnpm compile si tu utilises TypeScript
# pnpm compile

vsce package || { echo \"❌ Échec du packaging\"; exit 1; }

# 4. Installation de l’extension
VSIX_FILE=$(ls *.vsix | head -n 1)
"$VSCODE_PATH" --install-extension "$VSIX_FILE" || {
    echo "❌ Échec de l’installation de l’extension"
    exit 1
}

echo "✅ Extension RHL installée avec succès dans VSCode 🎉"
