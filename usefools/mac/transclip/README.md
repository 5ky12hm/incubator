# transclip

translate clipboard

## Usage

```
% uv run transclip.py
```

## Done notes

- setup python version
    ```
    % nv ./pyproject.toml
    requires-python = "==3.12.8"
    % uv python install 3.12.8
    % uv python pin 3.12.8
    ```

- install python package
    ```
    % uv add pyperclip
    % uv add googletrans
    % uv add pyobjc
    ```

- update googletrans version
    ```
    % nv ./pyproject.toml
    "googletrans>=3.0.0",
    to
    "googletrans>=4.0.0rc1",
    ```
