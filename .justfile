new year day:
  @echo Creating solution for {{year}} {{day}}
  @cp template.rs advent{{year}}/src/day{{day}}.rs
  @printf "\n\n[[bin]]\nname = \"2022-day{{day}}\"\npath = \"src/day{{day}}.rs\"" >> advent{{year}}/Cargo.toml
  @sed -i 's/YEAR/{{year}}/g' advent{{year}}/src/day{{day}}.rs
  @sed -i 's/DAY/{{day}}/g' advent{{year}}/src/day{{day}}.rs
