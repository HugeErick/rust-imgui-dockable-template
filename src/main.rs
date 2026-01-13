mod support;
use rfd::FileDialog;
use std::path::PathBuf;

fn main() {
  let system = support::init(file!());

  let mut selected_path: Option<PathBuf> = None;

  system.main_loop(move |_, ui| {
    ui.dockspace_over_main_viewport();
    ui.window("Hello world")
      .size([300.0, 300.0], imgui::Condition::FirstUseEver)
      .build(|| {
        ui.text("Hello world!");
        ui.separator();
        let mouse_pos = ui.io().mouse_pos;
        ui.text(format!("Mouse Position: ({:.1},{:.1})", mouse_pos[0], mouse_pos[1]));
        ui.separator();


        // file dialog
        if ui.button("Select file") && let Some(path) = FileDialog::new().pick_file() {
          selected_path = Some(path);
        }


        // folder dialog
        if ui.button("Select Folder") && let Some(path) = FileDialog::new().pick_folder() {
          selected_path = Some(path);
        }

        // display dialogs result
        if let Some(ref path) = selected_path {
          ui.text(format!("Selected: {}", path.display()));
          if ui.button("Clear") {
            selected_path = None;
          }
        } else {
          ui.text("No file or folder selected.");
        }

      });

  });
}
