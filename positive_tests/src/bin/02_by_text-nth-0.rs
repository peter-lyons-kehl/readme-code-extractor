mce::nth!(
    r##"
    markdown_file_path = "00.md"
    start_prefix = "#[::no_panic::no_panic] fn main() {"
    
    final_suffix = "}"
    "##
    @ 0
);
