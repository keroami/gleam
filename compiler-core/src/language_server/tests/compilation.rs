use lsp_types::{
    CodeAction, CodeActionContext, CodeActionParams, PartialResultParams, Position,
    TextDocumentIdentifier, WorkDoneProgressParams,
};

use url::Url;

use super::*;

#[test]
fn compile_please() {
    let io = LanguageServerTestIO::new();
    let mut engine = setup_engine(&io);

    let response = engine.compile_please();
    assert!(response.result.is_ok());
    assert!(response.warnings.is_empty());
    assert!(response.compiled_modules.is_empty());

    drop(engine);
    let actions = io.into_actions();
    assert_eq!(
        actions,
        vec![
            // new
            Action::DependencyDownloadingStarted,
            Action::DownloadDependencies,
            Action::DependencyDownloadingFinished,
            Action::LockBuild,
            Action::UnlockBuild,
            // compile_please
            Action::CompilationStarted,
            Action::LockBuild,
            Action::UnlockBuild,
            Action::CompilationFinished,
        ]
    )
}

#[test]
fn compile_error_in_src() {
    let io = LanguageServerTestIO::new();
    let mut engine = setup_engine(&io);

    io.src_module("app/error", "pub type Error {");

    let response = engine.compile_please();
    assert!(response.result.is_err());
    assert!(response.warnings.is_empty());
    assert!(response.compiled_modules.is_empty());

    drop(engine);
    let actions = io.into_actions();
    assert_eq!(
        actions,
        vec![
            // new
            Action::DependencyDownloadingStarted,
            Action::DownloadDependencies,
            Action::DependencyDownloadingFinished,
            Action::LockBuild,
            Action::UnlockBuild,
            // compile_please
            Action::CompilationStarted,
            Action::LockBuild,
            Action::UnlockBuild,
            Action::CompilationFinished,
        ]
    )
}

#[test]
fn compile_error_in_test() {
    let io = LanguageServerTestIO::new();
    let mut engine = setup_engine(&io);

    io.test_module("app/error", "pub type Error {");

    let response = engine.compile_please();
    assert!(response.result.is_err());
    assert!(response.warnings.is_empty());
    assert!(response.compiled_modules.is_empty());

    drop(engine);
    let actions = io.into_actions();
    assert_eq!(
        actions,
        vec![
            // new
            Action::DependencyDownloadingStarted,
            Action::DownloadDependencies,
            Action::DependencyDownloadingFinished,
            Action::LockBuild,
            Action::UnlockBuild,
            // compile_please
            Action::CompilationStarted,
            Action::LockBuild,
            Action::UnlockBuild,
            Action::CompilationFinished,
        ]
    )
}

#[test]
fn make_fn_public() {
    let io = LanguageServerTestIO::new();
    let mut engine = setup_engine(&io);

    io.src_module("app/refactor", "fn main() { todo }");

    let response = engine.code_action(CodeActionParams {
        text_document: TextDocumentIdentifier {
            uri: Url::parse("file://src/app/refactor/bla.gleam").unwrap(),
        },
        range: lsp_types::Range {
            start: Position {
                line: 0,
                character: 0,
            },
            end: Position {
                line: 0,
                character: 0,
            },
        },
        context: CodeActionContext {
            diagnostics: vec![],
            only: None,
        },
        work_done_progress_params: WorkDoneProgressParams {
            work_done_token: None,
        },
        partial_result_params: PartialResultParams {
            partial_result_token: None,
        },
    });
    assert_eq!(
        response.result,
        Ok(Some(vec!(lsp_types::CodeActionOrCommand::CodeAction(
            CodeAction {
                title: "wibble".to_string(),
                kind: None,
                diagnostics: None,
                edit: None,
                command: None,
                is_preferred: None,
                disabled: None,
                data: None
            }
        ))))
    );
    assert!(response.warnings.is_empty());
    assert!(response.compiled_modules.is_empty());

    drop(engine);
    let actions = io.into_actions();
    assert_eq!(
        actions,
        vec![
            // new
            Action::DependencyDownloadingStarted,
            Action::DownloadDependencies,
            Action::DependencyDownloadingFinished,
            Action::LockBuild,
            Action::UnlockBuild,
            // compile_please
            Action::CompilationStarted,
            Action::LockBuild,
            Action::UnlockBuild,
            Action::CompilationFinished,
        ]
    )
}
