Manage dotfiles using filename conventions and templates.

Filename prefixes:
* `private_` — Set 0600 permissions (sensitive files)
* `executable_` — Set 0755 permissions (scripts)
* `dot_` — Convert to hidden file (`~/.name`)
* `dot_config/` — Config directory (`~/.config/`)
* `xdg_config/` — Cross-platform config
* `xdg_data/` — Cross-platform data
* `xdg_cache/` — Cross-platform cache
* `.tmpl` — Template file (Tera/Jinja2 syntax)

Behavior:
* Default mode is preview (dry-run), use `--apply` to make changes
* `--dir` and `--apply` cannot be used together
* Use `--verbose` to see detailed processing information

Examples:
1. Create template from existing config:
   cbp dot ~/.bashrc --dir ~/dotfiles/

2. Preview template:
   cbp dot ~/dotfiles/dot_bashrc.tmpl

3. Apply template:
   cbp dot -a ~/dotfiles/dot_bashrc.tmpl

4. Apply all templates:
   ```
   for f in ~/dotfiles/dot_*; do cbp dot -a "$f"; done
   ```

5. Preview with verbose output:
   cbp dot -v ~/dotfiles/dot_bashrc.tmpl