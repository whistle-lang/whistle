# Preprocessor

1. Lex full file
2. Find all imports
3. Lex imports then insert on the top
4. When lexing any function in the imported file, prefix with `__filename_`
5. Rename all imported "resources"
