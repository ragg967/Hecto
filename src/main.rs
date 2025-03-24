#![warn(clippy::all, clippy::permissions_set_readonly_false)]
mod editor;
use editor::Editor;

fn main() {
    let editor = Editor::default();
    editor.run();
}
