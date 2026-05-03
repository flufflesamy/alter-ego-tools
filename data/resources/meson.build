# Blueprint
blueprints = custom_target(
  'blueprints',
  input: files(
    'ui/content.blp',
    'ui/description.blp',
    'ui/possibility.blp',
    'ui/preferences.blp',
    'ui/procedural.blp',
    'ui/shortcuts.blp',
    'ui/sidebar.blp',
    'ui/window.blp',
  ),
  output: '.',
  command: [
    find_program('blueprint-compiler'),
    'batch-compile',
    '--minify', '@OUTPUT@',
    '@CURRENT_SOURCE_DIR@',
    '@INPUT@',
  ],
)

# Resources
resources = gnome.compile_resources(
  'resources',
  'resources.gresource.xml',
  gresource_bundle: true,
  source_dir: meson.current_build_dir(),
  install: true,
  install_dir: pkgdatadir,
  dependencies: blueprints,
)