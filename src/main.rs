extern crate gtk;

use std::process::Command;
use std::thread;

use gtk::{Application, ApplicationWindow, Box, Button, Entry, FileChooserAction, Label, Orientation, ResponseType, Spinner};
use gtk::prelude::*;

fn main() {
    gtk::init().expect("Failed to initialize GTK.");

    let app = Application::builder()
        .application_id("me._12emin34.vortex")
        .build();

    app.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("Vortex");
        window.set_default_size(400, 400);

        let vbox = Box::new(Orientation::Vertical, 10);

        let url_label = Label::new(Some("URL"));
        let url_entry = Entry::new();
        vbox.pack_start(&url_label, false, false, 0);
        vbox.pack_start(&url_entry, true, true, 0);

        let path_label = Label::new(Some("Path"));
        let path_entry = Entry::new();
        let select_button = Button::with_label("Select");
        vbox.pack_start(&path_label, false, false, 0);
        vbox.pack_start(&path_entry, true, true, 0);
        vbox.pack_start(&select_button, false, false, 0);

        let start_download_button = Button::with_label("Start");
        let download_spinner = Spinner::new();
        vbox.pack_start(&start_download_button, false, false, 0);
        vbox.pack_start(&download_spinner, true, true, 0);

        window.add(&vbox);

        window.show_all();

        // I absolutely freaking hate the clone here, but nothing I can do about it
        let path_entry_clone = path_entry.clone();
        select_button.connect_clicked(move |_| {
            let save_path_chooser = gtk::FileChooserDialog::with_buttons(
                Some("Select path to save files to"),
                Some(&window),
                FileChooserAction::SelectFolder,
                &[("Cancel", ResponseType::Cancel), ("Open", ResponseType::Accept)],
            );
            match save_path_chooser.run() {
                ResponseType::Accept => {
                    path_entry_clone.set_text(save_path_chooser.filename().unwrap().to_str().unwrap())
                }
                _ => (),
            };
            save_path_chooser.close();
        });

        // This is even worse, but whatever
        let path_entry_clone = path_entry.clone();
        let download_spinner_clone = download_spinner.clone();

        start_download_button.connect_clicked(move |_| {
            let url = url_entry.text();
            let path = path_entry_clone.text();
            download_spinner_clone.start();
            let download_handler = thread::spawn(move || {
                let output = Command::new("yt-dlp")
                    .arg("-o")
                    .arg(format!("{path}/%(title)s.%(ext)s"))
                    .arg("--extract-audio")
                    .arg("--audio-format")
                    .arg("mp3")
                    .arg(url.as_str())
                    .output()
                    .expect("Failed to execute command");

                let success_message_dialog = gtk::MessageDialog::builder().text("Download finished!").build();
                let fail_message_dialog = gtk::MessageDialog::builder().text("Download failed!").build();

                if output.status.success() {
                    println!("Success: {}", String::from_utf8_lossy(&output.stdout));
                    success_message_dialog.show_all();
                } else {
                    eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
                    fail_message_dialog.show_all();
                }
            });
            download_spinner_clone.stop();
        });
    });

    app.run();
}
