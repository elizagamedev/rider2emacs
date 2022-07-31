# rider2emacs

## What?

rider2emacs is a command-line utility which translates JetBrains Rider invocations
to emacsclient invocations.

## Why?

The Unity game engine editor only officially supports four IDEs/editors:
MonoDevelop, Visual Studio, Visual Studio Code, and JetBrains Rider. *If and
only if* the Unity editor is configured to open source files in one of these
editors will it generate sln and csproj files for your code. If you want to use
any other editor, including the best text editor, Emacs, with the OmniSharp LSP,
you must convince Unity that you're using one of the four editors listed above.

It gets worse: support for each of the above editors each generate
project/solution files completely differently from one another, and the ability
for OnniSharp to correctly interpret these project and solution files varies
just as much. The best of the bunch seems to be the Rider project generation.

## Why not just use a shell script or something?

Making this a shell script would more or less make this impossible for Windows
users. My first shot at this was done in C for portability reasons and to reduce
the dependency tree. However, the amount of `*printf*` calls, string
manipulation, and platform-specific code required for what is on paper a very
simple shim made me uncomfortable. Unfortunately, this means that installation
of [unity.el](https://github.com/elizagamedev/unity.el) is slightly more complex
than I would have liked.

## See Also

- <https://github.com/elizagamedev/unity.el>
- <https://eliza.sh/2021-06-01-using-unity-editor-with-emacs.html>
