### GitHub Copilot Commands
## Prompt: ?? find all large rust target folders and get size and location
## CMD:  find . -name "target" -type d -exec du -sh {} \;

## Prompt:  ?? delete all rust build directories to save space
## CMD: find . -name "target" -type d -exec rm -rf {} \;