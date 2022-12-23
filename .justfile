new year day:
  @echo Creating solution for {{year}} {{day}}
  @cp template.rs advent{{year}}/src/day{{day}}.rs
  @printf "\n\n[[bin]]\nname = \"2022-day{{day}}\"\npath = \"src/day{{day}}.rs\"" >> advent{{year}}/Cargo.toml
  @sed -i 's/YEAR/{{year}}/g' advent{{year}}/src/day{{day}}.rs
  @sed -i 's/DAY/{{day}}/g' advent{{year}}/src/day{{day}}.rs

clippy year day:
  cargo clippy --bin {{year}}-day{{day}} -- -Wclippy::pedantic -Aclippy::implicit_return -Aclippy::unwrap_used -Aclippy::missing_docs_in_private_items -Aclippy::expect_used -Aclippy::non_ascii_literal -Aclippy::shadow_reuse -Aclippy::shadow_same -Aclippy::indexing_slicing -Aclippy::unused_async -Aclippy::cast-possible-truncation -Aclippy::cast_sign_loss -Aclippy::doc_markdown -Aclippy::missing-panics-doc -Aclippy::missing-errors-doc -Aclippy::enum-glob-use
