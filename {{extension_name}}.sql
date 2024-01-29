-- Create both functions that make up the extension
CREATE FUNCTION foo AS WASM FROM LOCAL INFILE "{{extension_name}}.wasm" WITH WIT FROM LOCAL INFILE "{{extension_name}}.wit";
