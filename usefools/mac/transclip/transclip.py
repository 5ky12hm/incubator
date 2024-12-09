import os
import time
import pyperclip
from googletrans import Translator
from AppKit import NSWorkspace


class Transclip:
    allow_appnames = [
        "WezTerm",
    ]
    srcLang = "en"
    destLang = "ja"

    def run(self):
        translator = Translator()
        workspace = NSWorkspace.sharedWorkspace()
        log, sentence = None, None

        while True:
            while sentence is None or sentence == log:
                time.sleep(0.1)

                sentence = pyperclip.paste()
                if sentence is None:
                    continue

                if log is None:
                    log = sentence
                    continue

            log = sentence

            appname = workspace.activeApplication()["NSApplicationName"]
            if appname not in self.allow_appnames:
                print(appname)
                continue

            detected = translator.detect(sentence)
            if detected.lang != self.srcLang:
                print(detected)
                continue

            translated = translator.translate(
                sentence, src=self.srcLang, dest=self.destLang).text

            os.system(f"osascript -e 'display notification \"{translated}\"'")


if __name__ == "__main__":
    Transclip().run()
