readme_code_extractor::all!(
    r##"
    markdown_file_path = "simple_no_panic.md"
    start_prefix = "#[::no_panic::no_panic] fn main() {"
    
    final_suffix = "}"
    "##
);
