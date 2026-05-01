readme_code_extractor::nth!(
    r##"
    markdown_file_path = "01.md"
    start_prefix = "#[::no_panic::no_panic] fn main() {"
    
    final_suffix = "}"
    "##
    @ 1
);
