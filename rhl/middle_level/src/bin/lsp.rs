use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};
use std::fs;
use std::path::Path;

use middle_level::types::{Format, TypesMapping};
use middle_level::linter::DslLinter;

struct Backend {
    client: Client,
    formats: Vec<Format>,
    type_mappings: TypesMapping,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> tower_lsp::jsonrpc::Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client.log_message(MessageType::INFO, "RHL LSP prêt 🚀").await;
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        self.handle_lint(params.text_document.uri, params.text_document.text).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        if let Some(change) = params.content_changes.get(0) {
            self.handle_lint(params.text_document.uri, change.text.clone()).await;
        }
    }

    async fn shutdown(&self) -> tower_lsp::jsonrpc::Result<()> {
        Ok(())
    }
}

impl Backend {
    async fn handle_lint(&self, uri: Url, content: String) {
        let linter = DslLinter::new(&self.formats, &self.type_mappings);
        let errors = linter.lint(&content);

        let diagnostics: Vec<Diagnostic> = errors
            .into_iter()
            .map(|e| Diagnostic {
                range: Range {
                    start: Position::new((e.line - 1) as u32, e.column as u32),
                    end: Position::new((e.line - 1) as u32, (e.column + 1) as u32),
                },
                severity: Some(DiagnosticSeverity::ERROR),
                code: Some(NumberOrString::String(e.code)),
                message: e.message,
                source: Some("rhl-linter".into()),
                ..Default::default()
            })
            .collect();

        self.client.publish_diagnostics(uri, diagnostics, None).await;
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let ron_path = manifest_dir.join("functions.ron");
    let types_path = manifest_dir.join("types_mapping.ron");

    let format_str = fs::read_to_string(ron_path)?;
    let types_str = fs::read_to_string(types_path)?;

    let formats: Vec<Format> = ron::from_str(&format_str)?;
    let type_mappings: TypesMapping = ron::from_str(&types_str)?;

    let (service, socket) = LspService::new(|client| Backend {
        client,
        formats,
        type_mappings,
    });

    Server::new(stdin, stdout, socket).serve(service).await;
    Ok(())
}
