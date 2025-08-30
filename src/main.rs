use clap::{Arg, Command};
use colored::*;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

#[derive(Debug)]
struct FileStats {
    file_path: String,
    lines: usize,
    file_type: String,
    comments: usize,
    blanks: usize,
    characters: usize,
}

fn main() {
    let matches = Command::new("xYc")
        .about("统计XML和YANG文件的行数、字符数、注释行和空白行")
        .version("1.0")
        .arg(
            Arg::new("path")
                .short('p')
                .long("path")
                .value_name("PATH")
                .help("要统计的目录或文件路径（默认为当前目录）")
                .default_value("."),
        )
        .arg(
            Arg::new("recursive")
                .short('r')
                .long("recursive")
                .help("递归搜索子目录")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("detailed")
                .short('d')
                .long("detailed")
                .help("显示每个文件的详细信息")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("file-type")
                .short('t')
                .long("type")
                .value_name("TYPE")
                .help("指定文件类型 (xml, yang, all)")
                .value_parser(["xml", "yang", "all"])
                .default_value("all"),
        )
        .get_matches();

    let path = matches.get_one::<String>("path").unwrap();
    let recursive = matches.get_flag("recursive");
    let detailed = matches.get_flag("detailed");
    let file_type = matches.get_one::<String>("file-type").unwrap();

    match analyze_path(path, recursive, file_type) {
        Ok(results) => {
            display_results(&results, detailed);
        }
        Err(e) => {
            eprintln!("{} {}", "错误:".bright_red().bold(), e);
            std::process::exit(1);
        }
    }
}

fn analyze_path(
    path: &str,
    recursive: bool,
    file_type_filter: &str,
) -> Result<Vec<FileStats>, Box<dyn std::error::Error>> {
    let path = Path::new(path);
    let mut results = Vec::new();

    if path.is_file() {
        if let Some(stats) = analyze_file(path, file_type_filter)? {
            results.push(stats);
        }
    } else if path.is_dir() {
        if recursive {
            for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
                if entry.file_type().is_file() {
                    if let Some(stats) = analyze_file(entry.path(), file_type_filter)? {
                        results.push(stats);
                    }
                }
            }
        } else {
            for entry in fs::read_dir(path)? {
                let entry = entry?;
                if entry.file_type()?.is_file() {
                    if let Some(stats) = analyze_file(&entry.path(), file_type_filter)? {
                        results.push(stats);
                    }
                }
            }
        }
    } else {
        return Err(format!("路径不存在或无法访问: {}", path.display()).into());
    }

    Ok(results)
}

fn analyze_file(
    path: &Path,
    file_type_filter: &str,
) -> Result<Option<FileStats>, Box<dyn std::error::Error>> {
    let extension = path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("")
        .to_lowercase();

    let file_type = match extension.as_str() {
        "xml" => "XML",
        "yang" => "YANG",
        _ => return Ok(None),
    };

    // 检查文件类型过滤器
    match file_type_filter {
        "xml" if file_type != "XML" => return Ok(None),
        "yang" if file_type != "YANG" => return Ok(None),
        _ => {}
    }

    let content = fs::read_to_string(path)?;
    let lines: Vec<&str> = content.lines().collect();
    let total_lines = lines.len();
    let characters = content.chars().count();

    let mut comments = 0;
    let mut blanks = 0;

    for line in &lines {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            blanks += 1;
        } else if (file_type == "XML" && (trimmed.starts_with("<!--") || trimmed.contains("<!--")))
            || (file_type == "YANG" && (trimmed.starts_with("//") || trimmed.starts_with("/*")))
        {
            comments += 1;
        }
    }

    Ok(Some(FileStats {
        file_path: path.display().to_string(),
        lines: total_lines,
        file_type: file_type.to_string(),
        comments,
        blanks,
        characters,
    }))
}

fn display_results(results: &[FileStats], detailed: bool) {
    if results.is_empty() {
        println!("{}", "未找到匹配的文件".bright_yellow());
        return;
    }

    // 显示详细信息（如果需要）
    if detailed {
        println!("\n{}", "详细文件列表:".bright_green().bold());
        println!("{}", "━".repeat(95).bright_blue());
        println!(
            " {:<45} {:<8} {:<8} {:<10} {:<8} {:<8}",
            "File Path".bright_white().bold(),
            "Type".bright_white().bold(),
            "Lines".bright_white().bold(),
            "Characters".bright_white().bold(),
            "Comments".bright_white().bold(),
            "Blanks".bright_white().bold()
        );
        println!("{}", "━".repeat(95).bright_blue());

        for stats in results {
            let type_color = match stats.file_type.as_str() {
                "XML" => stats.file_type.bright_yellow(),
                "YANG" => stats.file_type.bright_cyan(),
                _ => stats.file_type.white(),
            };

            println!(
                " {:<45} {:<8} {:<8} {:<10} {:<8} {:<8}",
                stats.file_path.to_string().bright_white(),
                type_color,
                stats.lines.to_string().bright_magenta(),
                stats.characters.to_string().bright_cyan(),
                stats.comments.to_string().bright_yellow(),
                stats.blanks.to_string().bright_blue()
            );
        }
        println!("{}", "━".repeat(95).bright_blue());
    }

    // 计算统计数据
    let mut language_stats = std::collections::HashMap::new();
    let mut total_files = 0;
    let mut total_lines = 0;
    let mut total_characters = 0;
    let mut total_comments = 0;
    let mut total_blanks = 0;

    for stats in results {
        let entry = language_stats
            .entry(stats.file_type.clone())
            .or_insert((0, 0, 0, 0, 0));
        entry.0 += 1; // files
        entry.1 += stats.lines; // lines
        entry.2 += stats.characters; // characters
        entry.3 += stats.comments; // comments
        entry.4 += stats.blanks; // blanks

        total_files += 1;
        total_lines += stats.lines;
        total_characters += stats.characters;
        total_comments += stats.comments;
        total_blanks += stats.blanks;
    }

    // 显示统计表格
    println!("\n{}", "统计摘要:".bright_green().bold());
    println!("{}", "━".repeat(85).bright_blue());
    println!(
        " {:<12} {:<12} {:<12} {:<12} {:<12} {:<12}",
        "Language".bright_white().bold(),
        "Files".bright_white().bold(),
        "Lines".bright_white().bold(),
        "Characters".bright_white().bold(),
        "Comments".bright_white().bold(),
        "Blanks".bright_white().bold()
    );
    println!("{}", "━".repeat(85).bright_blue());

    // 按语言排序显示
    let mut sorted_langs: Vec<_> = language_stats.iter().collect();
    sorted_langs.sort_by_key(|(lang, _)| *lang);

    for (language, (files, lines, characters, comments, blanks)) in sorted_langs {
        let lang_color = match language.as_str() {
            "XML" => language.bright_yellow(),
            "YANG" => language.bright_cyan(),
            _ => language.white(),
        };

        println!(
            " {:<12} {:<12} {:<12} {:<12} {:<12} {:<12}",
            lang_color,
            files.to_string().bright_white(),
            lines.to_string().bright_white(),
            characters.to_string().bright_cyan(),
            comments.to_string().bright_yellow(),
            blanks.to_string().bright_blue()
        );
    }

    println!("{}", "━".repeat(85).bright_blue());
    println!(
        " {:<12} {:<12} {:<12} {:<12} {:<12} {:<12}",
        "Total".bright_white().bold(),
        total_files.to_string().bright_white().bold(),
        total_lines.to_string().bright_white().bold(),
        total_characters.to_string().bright_cyan().bold(),
        total_comments.to_string().bright_yellow().bold(),
        total_blanks.to_string().bright_blue().bold()
    );
    println!("{}", "━".repeat(85).bright_blue());
}
