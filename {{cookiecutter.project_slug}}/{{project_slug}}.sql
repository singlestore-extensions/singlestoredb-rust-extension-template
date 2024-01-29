-- Create both functions that make up the extension
CREATE FUNCTION foo AS WASM FROM LOCAL INFILE "{{cookiecutter.project_slug}}.wasm" WITH WIT FROM LOCAL INFILE "{{cookiecutter.project_slug}}.wit";
