use fs_extra::copy_items;
use std::{
    env,
    fs::{self, File},
    io,
    path::Path,
    process::{exit, Command},
};

struct Theme {
    theme_name: String,
    vencord_path: String,
    pywal_discord_path: String,
    pywal_colors: String,
}

enum Args {
    Help,
    Theme,
    Reload,
    Install,
    Uninstall,
    Wall,
    Other,
}

impl Args {
    fn from_str(arg: &str) -> Args {
        match arg {
            "-h" | "--help" => Args::Help,
            "-t" | "--theme" => Args::Theme,
            "-r" | "--reload" => Args::Reload,
            "-i" | "--install" => Args::Install,
            "-w" | "--wall" => Args::Wall,
            "-u" | "--uninstall" => Args::Uninstall,
            _ => Args::Other,
        }
    }
}

impl Theme {
    fn help_menu() {
        println!("Usage: pywal-discord [command]\n");
        println!("Commands:");
        println!("  -h, --help                     Display this info");
        println!("  -i, --install                  Install");
        println!("  -u, --uninstall                Uninstall");
        println!("  -r, --reload                   Reload Theme");
        println!("  -t, --theme theme_name         Available: [default,abou]");
    }

    fn copy_files(files: Vec<&str>, dest_dir: &String) {
        let options = fs_extra::dir::CopyOptions::new();

        for file in files {
            let file_name = Path::new(file).file_name().unwrap().to_str().unwrap();
            let dest_path = Path::new(dest_dir).join(file_name);

            if !dest_path.exists() {
                match copy_items(&[file], dest_dir, &options) {
                    Ok(_) => {}
                    Err(err) => eprintln!("Error coping the files: {}", err),
                }
            }
        }
    }

    fn open_file(file: &String) -> File {
        match File::open(file) {
            Ok(f) => f,
            Err(_) => {
                eprintln!("Failed to opening file: {}", file);
                exit(1);
            }
        }
    }

    fn create_file(file: &String) -> File {
        match File::create(file) {
            Ok(f) => f,
            Err(_) => {
                eprintln!("Failed to creating file: {}", file);
                exit(1);
            }
        }
    }

    fn create_directory(path: &Path) {
        match fs::create_dir_all(path) {
            Ok(_) => {}
            Err(err) => eprintln!("Failed to create config directory: {}", err),
        };
    }

    fn reload(&self) {
        let pywal_discord_meta = format!(r"{}\meta.css", &self.pywal_discord_path);
        let pywal_discord_colors = format!(
            r"{}\pywal-discord-{}.css",
            self.pywal_discord_path, self.theme_name
        );

        let vencord_theme = format!(
            r"{}\pywal-discord-{}.css",
            self.vencord_path, self.theme_name
        );

        let mut pywal_discord_meta_file = Theme::open_file(&pywal_discord_meta);
        let mut colors = Theme::open_file(&self.pywal_colors);
        let mut pywal_discord_colors_file = Theme::open_file(&pywal_discord_colors);

        let mut vencord_theme_file = Theme::create_file(&vencord_theme);

        io::copy(&mut pywal_discord_meta_file, &mut vencord_theme_file)
            .expect("Failed to write file");
        io::copy(&mut colors, &mut vencord_theme_file).expect("Failed to write file");
        io::copy(&mut pywal_discord_colors_file, &mut vencord_theme_file)
            .expect("Failed to write file");
    }

    fn install(&self) {
        let path = Path::new(&self.pywal_discord_path);

        if !path.exists() {
            Theme::create_directory(path);
        }

        let files = vec![
            "./config/meta.css",
            "./config/pywal-discord-abou.css",
            "./config/pywal-discord-default.css",
        ];

        Theme::copy_files(files, &self.pywal_discord_path);
        self.reload();
    }

    fn uninstall(&self) {
        match fs::remove_dir_all(&self.pywal_discord_path) {
            Ok(_) => println!("Sucessfully removed {}", self.pywal_discord_path),
            Err(err) => eprintln!(
                "Failed to remove {}\nError: {}",
                self.pywal_discord_path, err,
            ),
        };

        let file = format!(
            r"{}\pywal-discord-{}.css",
            self.vencord_path, self.theme_name
        );

        match fs::remove_file(&file) {
            Ok(_) => println!("Sucessfully removed file {}", file),
            Err(err) => eprintln!("Failed to remove file {}\nError: {}", file, err),
        };
    }

    fn set_theme(&mut self, new_theme: String) {
        self.theme_name = new_theme;
    }

    /*
     * This function is a temporary measure.
     * This function will be moved when i make the other script to change the wallpaper
     */
    fn set_wall(&self, wall: &str) {
        let out = Command::new("wal")
            .args(["-i", wall, "-n"])
            .output()
            .expect("Failed to execute the command: wal");

        if !out.status.success() {
            let error = String::from_utf8_lossy(&out.stderr);
            println!("Erro: {}", error);
        }

        self.reload();
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // let username = env::var("USERNAME").expect("Couldn't get the environment user.");
    let username = String::from("rafa1");

    let mut theme = Theme {
        theme_name: String::from("default"),
        vencord_path: format!(r"C:\Users\{}\AppData\Roaming\Vencord\themes", username),
        pywal_discord_path: format!(r"C:\Users\{}\.config\pywal-discord", username),
        pywal_colors: format!(r"C:\Users\{}\.cache\wal\colors.css", username),
    };

    match args.get(1).map(|arg| Args::from_str(arg.as_str())) {
        Some(Args::Help) => Theme::help_menu(),
        Some(Args::Install) => theme.install(),
        Some(Args::Reload) => theme.reload(),
        Some(Args::Wall) => theme.set_wall(args[2].as_str()),
        Some(Args::Theme) => {
            if args.get(2).is_none() {
                eprintln!("Please enter a theme name.\n");
                Theme::help_menu();
                exit(1);
            }

            theme.set_theme(args[2].clone());
            theme.reload();
        }
        Some(Args::Uninstall) => theme.uninstall(),
        Some(Args::Other) => {
            eprintln!("Invalid argument.\n");
            Theme::help_menu();
            exit(1);
        }
        None => Theme::help_menu(),
    }
}
