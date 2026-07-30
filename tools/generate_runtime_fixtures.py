#!/usr/bin/env python3
"""Generate small real compiler/runtime error fixtures from local toolchains."""

import json
import shutil
import subprocess
import tempfile
from pathlib import Path


def run(command, cwd=None):
    result = subprocess.run(
        command,
        cwd=cwd,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        timeout=30,
        check=False,
    )
    return result.returncode, result.stdout


def main():
    output = Path("fixtures/generated/runtime")
    output.mkdir(parents=True, exist_ok=True)
    records = []
    with tempfile.TemporaryDirectory(prefix="ee-fixtures-") as temporary:
        root = Path(temporary)
        cases = [
            (
                "python-traceback",
                ["python3", "-c", "def fail():\n raise ValueError('bad input')\nfail()"],
                None,
            ),
            (
                "node-typeerror",
                ["node", "-e", "const value=null; console.log(value.name)"],
                None,
            ),
            (
                "shell-command-not-found",
                ["bash", "-c", "missing_ee_command"],
                None,
            ),
            (
                "postgres-connection",
                ["psql", "-h", "127.0.0.1", "-p", "1", "-d", "missing"],
                None,
            ),
            ("docker-cli", ["docker", "inspect", "missing-ee-container"], None),
            ("git-not-repository", ["git", "-C", temporary, "status"], None),
        ]
        sources = {
            "java-compile": (
                "Broken.java",
                "class Broken { String value = 42; }\n",
                ["java", "Broken.java"],
            ),
            "java-runtime": (
                "Crash.java",
                "class Crash { public static void main(String[] a) { throw new IllegalStateException(\"broken state\"); } }\n",
                ["java", "Crash.java"],
            ),
            "gcc-compile": (
                "broken.c",
                "int main(void) { return missing_symbol; }\n",
                ["gcc", "broken.c"],
            ),
            "gpp-compile": (
                "broken.cpp",
                "#include <string>\nint main() { std::string value = 42; }\n",
                ["g++", "broken.cpp"],
            ),
            "npm-missing-script": (
                "package.json",
                '{"name":"ee-fixture","version":"1.0.0","scripts":{}}\n',
                ["npm", "run", "missing"],
            ),
        }
        for name, (filename, source, command) in sources.items():
            directory = root / name
            directory.mkdir()
            (directory / filename).write_text(source, encoding="utf-8")
            cases.append((name, command, directory))
        for name, command, cwd in cases:
            if shutil.which(command[0]) is None:
                continue
            code, text = run(command, cwd)
            text = text.replace(temporary, "<TMP>")
            text = text.replace(str(Path.home()), "<HOME>")
            (output / f"{name}.log").write_text(text.rstrip() + "\n", encoding="utf-8")
            records.append({"id": name, "command": command, "exit_code": code})
    (output / "provenance.json").write_text(
        json.dumps(
            {
                "generator": "tools/generate_runtime_fixtures.py",
                "generated_locally": True,
                "fixtures": records,
            },
            indent=2,
        )
        + "\n",
        encoding="utf-8",
    )
    print(f"ok: {len(records)} runtime fixtures")


if __name__ == "__main__":
    main()
