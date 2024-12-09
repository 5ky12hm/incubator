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
    - to avoid the following error
        ```
        Traceback (most recent call last):
          File "/Users/hsayama/repo/social/incubator/usefools/mac/./transclip.py", line 6, in <module>
            from googletrans import Translator
          File "/Users/hsayama/repo/social/incubator/usefools/mac/.venv/lib/python3.13/site-packages/googletrans/__init__.py", line 6, in <module>
            from googletrans.client import Translator
          File "/Users/hsayama/repo/social/incubator/usefools/mac/.venv/lib/python3.13/site-packages/googletrans/client.py", line 11, in <module>
            import httpx
          File "/Users/hsayama/repo/social/incubator/usefools/mac/.venv/lib/python3.13/site-packages/httpx/__init__.py", line 2, in <module>
            from ._api import delete, get, head, options, patch, post, put, request, stream
          File "/Users/hsayama/repo/social/incubator/usefools/mac/.venv/lib/python3.13/site-packages/httpx/_api.py", line 3, in <module>
            from ._client import Client, StreamContextManager
          File "/Users/hsayama/repo/social/incubator/usefools/mac/.venv/lib/python3.13/site-packages/httpx/_client.py", line 8, in <module>
            from ._auth import Auth, BasicAuth, FunctionAuth
          File "/Users/hsayama/repo/social/incubator/usefools/mac/.venv/lib/python3.13/site-packages/httpx/_auth.py", line 10, in <module>
            from ._models import Request, Response
          File "/Users/hsayama/repo/social/incubator/usefools/mac/.venv/lib/python3.13/site-packages/httpx/_models.py", line 1, in <module>
            import cgi
        ModuleNotFoundError: No module named 'cgi'
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
    - to avoid the following error
        ```
        Traceback (most recent call last):
          File "/Users/hsayama/repo/social/incubator/usefools/mac/transclip/transclip.py", line 48, in <module>
            Transclip().run()
          File "/Users/hsayama/repo/social/incubator/usefools/mac/transclip/transclip.py", line 37, in run
            detected = translator.detect(sentence)
                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^
          File "/Users/hsayama/repo/social/incubator/usefools/mac/transclip/.venv/lib/python3.12/site-packages/googletrans/client.py", line 255, in detect
            data = self._translate(text, 'en', 'auto', kwargs)
                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
          File "/Users/hsayama/repo/social/incubator/usefools/mac/transclip/.venv/lib/python3.12/site-packages/googletrans/client.py", line 78, in _translate
            token = self.token_acquirer.do(text)
                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
          File "/Users/hsayama/repo/social/incubator/usefools/mac/transclip/.venv/lib/python3.12/site-packages/googletrans/gtoken.py", line 194, in do
            self._update()
          File "/Users/hsayama/repo/social/incubator/usefools/mac/transclip/.venv/lib/python3.12/site-packages/googletrans/gtoken.py", line 62, in _update
            code = self.RE_TKK.search(r.text).group(1).replace('var ', '')
                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        AttributeError: 'NoneType' object has no attribute 'group'
        ```

