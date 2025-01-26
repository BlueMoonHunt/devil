fn create_rust_project(
    project_path: &std::path::PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    let status = std::process::Command::new("cargo")
        .arg("init")
        .arg("--bin")
        .arg("--quiet")
        .current_dir(project_path)
        .status()?;

    if !status.success() {
        return Err("cargo init failed".into());
    }

    Ok(())
}

fn create_c_project(
    project_path: &std::path::PathBuf,
    project_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let src_path = project_path.join("src");
    std::fs::create_dir_all(&src_path)?;
    std::fs::write(
        src_path.join("main.c"),
        "#include <stdio.h>\n\nint main() {\n    printf(\"Hello, world!\\n\");\n    return 0;\n}\n",
    )?;
    std::fs::write(
        project_path.join("CMakeLists.txt"),
        format!(
            "cmake_minimum_required(VERSION 3.10)\n\
            project({} VERSION 0.1.0 LANGUAGES C)\n\n\
            set(CMAKE_C_FLAGS \"${{CMAKE_C_FLAGS}} -Wall -Wextra -Wpedantic\")\n\
            set(CMAKE_C_STANDARD 11)\n\
            set(CMAKE_C_STANDARD_REQUIRED ON)\n\
            set(CMAKE_C_FLAGS \"${{CMAKE_C_FLAGS}} -std=c${{CMAKE_C_STANDARD}}\")\n\n\
            file(GLOB_RECURSE SOURCES \"src/*.c\")\n\n\
            add_executable({} src/main.c ${{SOURCES}})\n\
            target_include_directories({} PRIVATE src)\n\
            set_target_properties({} PROPERTIES\n\
                RUNTIME_OUTPUT_DIRECTORY \"${{CMAKE_BINARY_DIR}}/bin\"\n\
                ARCHIVE_OUTPUT_DIRECTORY \"${{CMAKE_BINARY_DIR}}/lib\"\n\
                LIBRARY_OUTPUT_DIRECTORY \"${{CMAKE_BINARY_DIR}}/lib\"\n\
            )",
            project_name, project_name, project_name, project_name
        ),
    )?;
    Ok(())
}

fn create_cpp_project(
    project_path: &std::path::PathBuf,
    project_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let src_path = project_path.join("src");
    std::fs::create_dir_all(&src_path)?;
    std::fs::write(
        src_path.join("main.cpp"),
        "#include <iostream>\n\nint main() {{\n    std::cout << \"Hello, world!\" << std::endl;\n    return 0;\n}}\n",
    )?;
    std::fs::write(
        project_path.join("CMakeLists.txt"),
        format!(
            "cmake_minimum_required(VERSION 3.10)\n\
            project({} VERSION 0.1.0 LANGUAGES CXX)\n\n\
            set(CMAKE_CXX_FLAGS \"${{CMAKE_CXX_FLAGS}} -Wall -Wextra -Wpedantic\")\n\
            set(CMAKE_CXX_STANDARD 17)\n\
            set(CMAKE_CXX_STANDARD_REQUIRED ON)\n\
            set(CMAKE_CXX_FLAGS \"${{CMAKE_CXX_FLAGS}} -std=c++${{CMAKE_CXX_STANDARD}}\")\n\n\
            file(GLOB_RECURSE SOURCES \"src/*.cpp\")\n\n\
            add_executable({} ${{SOURCES}})\n\
            target_include_directories({} PRIVATE src)\n\
            set_target_properties({} PROPERTIES\n\
                RUNTIME_OUTPUT_DIRECTORY \"${{CMAKE_BINARY_DIR}}/bin\"\n\
                ARCHIVE_OUTPUT_DIRECTORY \"${{CMAKE_BINARY_DIR}}/lib\"\n\
                LIBRARY_OUTPUT_DIRECTORY \"${{CMAKE_BINARY_DIR}}/lib\"\n\
            )",
            project_name, project_name, project_name, project_name
        ),
    )?;
    Ok(())
}

fn create_project(project_name: &str, language: &str) -> Result<(), Box<dyn std::error::Error>> {
    let project_path = if project_name == "." {
        std::env::current_dir()?
    } else {
        std::path::PathBuf::from("Dev").join(project_name)
    };

    if project_path.exists() && project_name != "." {
        return Err(format!(
            "Project directory already exists: {}",
            project_path.display()
        )
        .into());
    }

    std::fs::create_dir_all(&project_path)?;

    match language {
        "rust" => create_rust_project(&project_path)?,
        "c" => create_c_project(&project_path, project_name)?,
        "c++" | "cpp" => create_cpp_project(&project_path, project_name)?,
        _ => return Err(format!("Language not supported: {}", language).into()),
    }

    println!("Created project at: {}", project_path.display());
    Ok(())
}

fn show_status(path: &str, ignore_list: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let path = if path == "." {
        std::env::current_dir()?
    } else {
        std::path::PathBuf::from(path)
    };

    fn print_files(
        path: &std::path::PathBuf,
        prefix: &str,
        ignore_list: &[String],
    ) -> std::io::Result<()> {
        if path.is_dir() {
            let mut entries: Vec<_> = std::fs::read_dir(path)?
                .filter_map(Result::ok)
                .map(|entry| entry.path())
                .filter(|path| {
                    path.file_name()
                        .and_then(|s| s.to_str())
                        .map_or(true, |file_name| {
                            !ignore_list.iter().any(|ignore| ignore == file_name)
                        })
                })
                .collect();

            entries.sort(); // Sort entries for consistent output

            let num_entries = entries.len();

            for (i, entry) in entries.iter().enumerate() {
                let file_name = entry.file_name().unwrap().to_str().unwrap();
                let is_last = i == num_entries - 1;

                println!(
                    "{}{}{}",
                    prefix,
                    if is_last { "└── " } else { "├── " },
                    file_name
                );

                if entry.is_dir() {
                    let new_prefix = format!("{}{}", prefix, if is_last { "    " } else { "│   " });
                    print_files(entry, &new_prefix, ignore_list)?;
                }
            }
        }
        Ok(())
    }

    print_files(&path, "", ignore_list)?;

    Ok(())
}

fn print_usage() {
    println!("Usage: devil <command> [options]");
    println!("Commands:");
    println!("  project <project_name> <project_language>");
    println!("  status <path> [--ignore <folder1> <folder2> ...]");
    println!("  help");
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        print_usage();
        return Ok(());
    }

    let command = &args[1];

    match command.as_str() {
        "project" => {
            if args.len() != 4 {
                eprintln!("Usage: devil project <project_name> <project_language>");
                std::process::exit(1);
            }
            let project_name = &args[2];
            let project_language = &args[3];
            create_project(project_name, project_language)?;
        }
        "status" => {
            if args.len() < 3 {
                eprintln!("Usage: devil status <path> [--ignore <folder1> <folder2> ...]");
                std::process::exit(1);
            }
            let path = &args[2];
            let mut ignore_list = Vec::new();
            let mut i = 3;
            while i < args.len() {
                if args[i] == "--ignore" {
                    i += 1;
                    while i < args.len() && !args[i].starts_with("--") {
                        ignore_list.push(args[i].clone());
                        i += 1;
                    }
                } else {
                    eprintln!("Invalid argument: {}", args[i]);
                    std::process::exit(1);
                }
            }
            show_status(path, &ignore_list)?;
        }
        "help" => print_usage(),
        "version" => {
            println!("devil v{}", std::env!("CARGO_PKG_VERSION"));
        }
        _ => {
            eprintln!("Invalid command: {}", command);
            print_usage();
            std::process::exit(1);
        }
    }
    Ok(())
}
